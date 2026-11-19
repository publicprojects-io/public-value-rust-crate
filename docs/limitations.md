# Limitations

- Outputs are deterministic calculations, not forecasts; input uncertainty remains the caller's
  responsibility.
- `Money` uses a single internal USD tag and performs no currency conversion.
- `Ratio` and `Percentage` use `f64`; callers should use tolerances for transcendental formulas.
- Several functions accept negative or greater-than-100% rates because the source material needs
  them. The crate does not silently clamp policy assumptions.
- Division functions panic on a zero denominator, as documented beside each API.
- Qualitative frameworks expose the decisions actually stated by their sources; they do not invent
  aggregate scores.
- Published policy thresholds and discount schedules can change. They remain parameters unless a
  source topic specifically defines a stable rule.
- Worked examples demonstrate formula fidelity but do not establish that a method is appropriate
  for every jurisdiction or decision.
