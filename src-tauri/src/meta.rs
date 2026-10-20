//! Metadades de pistes, àlbums i artistes amb el protobuf del client natiu
//! (`/extended-metadata/v0/extended-metadata`, per lots), via la sessió de librespot.
//! Retorna JSON amb la mateixa forma que espera la interfície (src/lib/api.ts).

use librespot::core::{session::Session, spotify_id::SpotifyId};
use librespot::protocol::{
    extended_metadata::{BatchedEntityRequest, EntityRequest, ExtensionQuery},
    extension_kind::ExtensionKind,
    metadata::{self, image::Size},
};
use protobuf::{EnumOrUnknown, Message};
use serde_json::{json, Value};

type Res<T> = Result<T, String>;
const BATCH: usize = 300;

fn b62(gid: &[u8]) -> String {
    SpotifyId::from_raw(gid).and_then(|id| id.to_base62()).unwrap_or_default()
}

fn hex(b: &[u8]) -> String {
    b.iter().map(|x| format!("{x:02x}")).collect()
}

/// (miniatura ~64 px, gran ~640 px) d'un grup d'imatges.
fn images(imgs: &[metadata::Image]) -> (Option<String>, Option<String>) {
    let url = |s: Size| {
        imgs.iter()
            .find(|i| i.size() == s)
            .map(|i| format!("https://i.scdn.co/image/{}", hex(i.file_id())))
    };
    let any = imgs.first().map(|i| format!("https://i.scdn.co/image/{}", hex(i.file_id())));
    (
        url(Size::SMALL).or_else(|| url(Size::DEFAULT)).or(any.clone()),
        url(Size::LARGE).or_else(|| url(Size::DEFAULT)).or(any),
    )
}

/// Demana `kind` per a totes les `uris`, en lots, i retorna els missatges en el mateix ordre.
async fn batch<M: Message>(session: &Session, kind: ExtensionKind, uris: &[String]) -> Res<Vec<M>> {
    let mut by_uri = std::collections::HashMap::new();
    for part in uris.chunks(BATCH) {
        let req = BatchedEntityRequest {
            entity_request: part
                .iter()
                .map(|u| EntityRequest {
                    entity_uri: u.clone(),
                    query: vec![ExtensionQuery { extension_kind: EnumOrUnknown::new(kind), ..Default::default() }],
                    ..Default::default()
                })
                .collect(),
            ..Default::default()
        };
        let res = session.spclient().get_extended_metadata(req).await.map_err(|e| e.to_string())?;
        for arr in res.extended_metadata {
            for d in arr.extension_data {
                if let Some(any) = d.extension_data.as_ref() {
                    if let Ok(m) = M::parse_from_bytes(&any.value) {
                        by_uri.insert(d.entity_uri.clone(), m);
                    }
                }
            }
        }
    }
    Ok(uris.iter().filter_map(|u| by_uri.remove(u)).collect())
}

fn artists_json(a: &[metadata::Artist]) -> Value {
    a.iter().map(|x| json!({ "id": b62(x.gid()), "name": x.name() })).collect()
}

fn track_json(t: &metadata::Track) -> Value {
    let al = &t.album;
    let (thumb, image) = images(&al.cover_group.image);
    let id = b62(t.gid());
    json!({
        "id": id,
        "uri": format!("spotify:track:{id}"),
        "name": t.name(),
        "artists": artists_json(&t.artist),
        "album": { "id": b62(al.gid()), "name": al.name(), "image": image, "thumb": thumb },
        "duration_ms": t.duration(),
        "explicit": t.explicit(),
    })
}

fn album_card(a: &metadata::Album) -> Value {
    let (_, image) = images(&a.cover_group.image);
    json!({
        "kind": "album",
        "id": b62(a.gid()),
        "name": a.name(),
        "subtitle": a.artist.iter().map(|x| x.name()).collect::<Vec<_>>().join(", "),
        "image": image,
    })
}

fn artist_card(a: &metadata::Artist) -> Value {
    let (_, image) = images(&a.portrait_group.image);
    json!({ "kind": "artist", "id": b62(a.gid()), "name": a.name(), "subtitle": "Artista", "image": image })
}

pub async fn tracks(s: &Session, uris: &[String]) -> Res<Value> {
    let uris: Vec<String> = uris.iter().filter(|u| u.starts_with("spotify:track:")).cloned().collect();
    let ts: Vec<metadata::Track> = batch(s, ExtensionKind::TRACK_V4, &uris).await?;
    Ok(ts.iter().map(track_json).collect())
}

pub async fn albums(s: &Session, uris: &[String]) -> Res<Value> {
    let a: Vec<metadata::Album> = batch(s, ExtensionKind::ALBUM_V4, uris).await?;
    Ok(a.iter().map(album_card).collect())
}

pub async fn artists(s: &Session, uris: &[String]) -> Res<Value> {
    let a: Vec<metadata::Artist> = batch(s, ExtensionKind::ARTIST_V4, uris).await?;
    Ok(a.iter().map(artist_card).collect())
}

/// Capçalera de l'àlbum i les seves pistes.
pub async fn album(s: &Session, id: &str) -> Res<Value> {
    let uri = format!("spotify:album:{id}");
    let a: metadata::Album = batch(s, ExtensionKind::ALBUM_V4, &[uri]).await?.pop().ok_or("Àlbum no trobat")?;
    let uris: Vec<String> = a
        .disc
        .iter()
        .flat_map(|d| &d.track)
        .map(|t| format!("spotify:track:{}", b62(t.gid())))
        .collect();
    Ok(json!({ "info": album_card(&a), "tracks": tracks(s, &uris).await? }))
}

/// Artista: capçalera, cançons populars al país del compte i discografia (àlbums i senzills).
pub async fn artist(s: &Session, id: &str) -> Res<Value> {
    let uri = format!("spotify:artist:{id}");
    let a: metadata::Artist = batch(s, ExtensionKind::ARTIST_V4, &[uri]).await?.pop().ok_or("Artista no trobat")?;
    let country = s.country();
    let top = a
        .top_track
        .iter()
        .find(|t| t.country() == country)
        .or_else(|| a.top_track.first())
        .map(|t| t.track.iter().map(|x| format!("spotify:track:{}", b62(x.gid()))).collect::<Vec<_>>())
        .unwrap_or_default();
    // Cada grup és un llançament (amb versions per país): n'hi ha prou amb el primer.
    let releases: Vec<String> = a
        .album_group
        .iter()
        .chain(&a.single_group)
        .filter_map(|g| g.album.first())
        .map(|x| format!("spotify:album:{}", b62(x.gid())))
        .take(30)
        .collect();
    let (_, image) = images(&a.portrait_group.image);
    Ok(json!({
        "info": { "kind": "artist", "id": id, "name": a.name(), "subtitle": "Artista", "image": image },
        "top": tracks(s, &top).await?,
        "albums": albums(s, &releases).await?,
    }))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn picks_image_sizes() {
        let img = |id: u8, s: Size| {
            let mut i = metadata::Image::new();
            i.set_file_id(vec![id]);
            i.set_size(s);
            i
        };
        let (thumb, big) = images(&[img(1, Size::DEFAULT), img(2, Size::SMALL), img(3, Size::LARGE)]);
        assert_eq!(thumb.as_deref(), Some("https://i.scdn.co/image/02"));
        assert_eq!(big.as_deref(), Some("https://i.scdn.co/image/03"));
        assert_eq!(images(&[]), (None, None));
    }
}
