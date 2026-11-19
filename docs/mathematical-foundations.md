# Mathematical foundations

The crate uses four recurring modelling shapes described in `spec/architecture.md`: direct
formulas, formulas with named intermediates, bundles of related metrics, and qualitative tests.

- **Foundations:** counterfactual value is observed change minus what would have happened anyway;
  future value is divided by `(1 + r)^t`; distributional weights scale benefits by relative income.
- **Economic appraisal:** NPSV subtracts present costs from present benefits; BCR divides them;
  cost effectiveness divides incremental cost by incremental outcomes.
- **Impact:** SROI applies deadweight, attribution, and drop-off before dividing present impact by
  inputs. Logic models preserve the causal chain rather than collapsing it into a score.
- **Performance:** ratios always expose their denominator; productivity adjusts outputs for quality
  before comparing them with inputs.
- **Philanthropy:** cost per outcome counts achieved outcomes, not service contacts; donor ROI and
  SROI keep funding leverage distinct from impact attribution.
- **Digital government:** transaction, channel-shift, platform, cyber-risk, and AI values are all
  explicit benefit-minus-cost or expected-loss calculations.
- **Societal indicators:** HDI and IHDI use geometric means so one strong dimension cannot fully
  compensate for a weak one; MPI is incidence multiplied by intensity.
- **Delivery:** Little's Law connects work in progress, throughput, and cycle time; cost of delay
  turns delivery time into foregone public value.

The rustdoc on every function gives the exact formula, units, panic conditions, and source worked
example. It is canonical when this overview and an implementation detail appear to differ.
