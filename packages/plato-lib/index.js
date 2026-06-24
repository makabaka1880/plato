/**
 * plato-lib — natural deduction proof engine (wasm).
 *
 * Usage:
 *   import init, { Formula, Ctx, Prover } from 'plato-lib';
 *   await init();
 *
 *   const a = Formula.atom('A');
 *   const ctx = Ctx.new();
 *   ctx.insert(a);
 *   const j = Prover.var_intro(ctx, a);
 *   console.log(j?.to_string()); // {A} ⊢ A
 */

export { default } from './pkg/plato_lib.js';
export * from './pkg/plato_lib.js';
