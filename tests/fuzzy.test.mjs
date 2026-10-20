// npm test (node --test). Importa el .ts directament: Node 22+ treu els tipus sol.
import { test } from "node:test";
import assert from "node:assert/strict";
import { matches } from "../src/lib/fuzzy.ts";

test("ignora accents i majúscules", () => {
  assert.ok(matches("cancons", "Cançons que m'agraden"));
  assert.ok(matches("CATALA", "Català"));
});

test("admet una lletra equivocada per paraula", () => {
  assert.ok(matches("oqes grasses", "Elefants · Oques Grasses"));
  assert.ok(matches("rosalia", "ROSALÍA"));
  assert.ok(matches("gdokeo", "GodKeo")); // dues lletres girades = un sol error
});

test("no troba coses que no hi són", () => {
  assert.ok(!matches("jazz", "Nice vibes"));
  assert.ok(!matches("abc", "Abdo")); // paraules curtes: exactes
  assert.ok(matches("", "qualsevol cosa"));
});
