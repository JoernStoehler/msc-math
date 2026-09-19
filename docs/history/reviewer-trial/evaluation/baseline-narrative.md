# Review of “Choosing and changing examples”

I assessed the supplied excerpt for an MSc mathematician familiar with EHZ capacity, systolic ratio and Lagrangian products. I have not treated extraction artifacts in symbols or layout as writing problems.

## Concerns

1. **The evidence for selection is too compressed to assess.**

   > “Tests on fresh candidates showed that low-ridge selection could raise the ratios relative to controls.”

   The reader is not told what the controls were, how low-ridge candidates were selected, or whether “raise the ratios” means a higher mean, a higher maximum, or another comparison. This matters because the paragraph establishes the success that motivates the transition from choosing examples to changing them. Unlike the subsequent experiments, it provides no concrete comparison by which the reader can understand the claimed success.

   **Confidence: high**, if this is the only account of those tests; lower if the preceding text supplies their design and results.

2. **The numerical-accuracy qualification is not connected to the reported effects.**

   > “The new experiments bound the error in capacity, but their volume calculation still uses floating-point arithmetic.”

   No size of the capacity error is given here, nor an indication of whether it is small compared with the later mean increase of 0.0153 and the changes used to classify eleven pairs as improved and five as worse. The qualification alerts the reader to uncertainty but leaves them unable to interpret its practical importance. The issue is the missing connection between computational accuracy and the stated comparisons, rather than a demand for a statistical significance test or a fully certified volume computation.

   **Confidence: medium.** An earlier numerical-methods section may already make the accuracy clear.

## Effective passages worth retaining

1. > “Thus a k × m sample means a product with k sides in the first factor and m in the second; the rejection rule is part of its distribution.”

   This ties the notation to the actual sampling procedure and makes the consequence of rejection explicit without requiring statistical vocabulary.

2. > “That success supplied a way to choose examples. It did not yet tell us which change to make to a given body.”

   This clearly explains why the next experiment is needed. It distinguishes an observed association useful for selection from evidence about how a particular body responds to a change.

3. > “Rotation improved three of the four starting bodies. Nevertheless, fresh sampling produced the larger maximum in every comparison.”

   This gives the reader the two relevant outcomes directly: rotation can improve a starting example, while fresh sampling can still be the better use of a fixed evaluation budget. The following numerical example makes that distinction concrete.

## Missing context

An earlier account of the low-ridge selection experiment and its controls could resolve the first concern. A numerical-methods discussion giving capacity tolerances and the expected reliability of volume calculations could resolve the second. The phrase “within the AI-assisted exploration” also presupposes an account of what assistance was used; it is unobjectionable if that account is available in the surrounding chapter. No judgment about human acceptance follows from mathematical correctness.
