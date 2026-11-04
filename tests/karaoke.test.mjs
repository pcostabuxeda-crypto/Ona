// npm test (node --test). Importa el .ts directament: Node 22+ treu els tipus sol.
import { test } from "node:test";
import assert from "node:assert/strict";
import { estimateWords } from "../src/lib/karaoke.ts";

test("reparteix la línia segons la llargada de les paraules", () => {
  const w = estimateWords("Salto a l'abisme", 10000, 14000);
  assert.deepEqual(
    w.map((x) => x.text),
    ["Salto ", "a ", "l'abisme"],
  );
  assert.equal(w[0].t, 10000);
  assert.ok(w[1].t > w[0].t && w[2].t > w[1].t);
  assert.ok(w[2].t < 14000 * 0.99); // acaba abans de la línia següent
});

test("línies sense paraules o sense línia següent", () => {
  assert.deepEqual(estimateWords("   ", 0, 1000), []);
  const w = estimateWords("hola", 5000);
  assert.equal(w.length, 1);
  assert.equal(w[0].t, 5000);
});
