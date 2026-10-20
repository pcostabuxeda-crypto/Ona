// npm test (node --test). Importa el .ts directament: Node 22+ treu els tipus sol.
import { test } from "node:test";
import assert from "node:assert/strict";
import { nextStep, shuffleRest, unshuffleRest } from "../src/lib/queue.ts";

test("l'aleatori no ressuscita el que ja ha sonat ni el que s'ha tret", () => {
  // Context de 6; han sonat 0 i 1 (pos 1); l'usuari ha tret el 4.
  const order = [0, 1, 2, 3, 5];
  let seed = 0.37;
  const rnd = () => (seed = (seed * 9301 + 49297) % 233280 / 233280);
  const s = shuffleRest(order, 1, rnd);
  assert.deepEqual(s.slice(0, 2), [0, 1]);
  assert.deepEqual([...s.slice(2)].sort(), [2, 3, 5]);
  assert.ok(!s.includes(4));
  assert.deepEqual(unshuffleRest([0, 1, 5, 2, 3], 1), [0, 1, 2, 3, 5]);
});

test("la cua de l'usuari va abans que el context, i el context continua on era", () => {
  assert.equal(nextStep(2, 3, 10, false), "user");
  assert.equal(nextStep(0, 3, 10, false), 4);
  assert.equal(nextStep(0, 9, 10, false), -1);
  assert.equal(nextStep(0, 9, 10, true), 0);
  assert.equal(nextStep(0, -1, 0, true), -1);
});
