---
name: algorithm-first
description: >
   Use this skill when solving programming problems involving algorithms,
   data structures, computational logic, or algorithmic exercises. Before
   implementing the solution, analyze and document the problem, inputs,
   outputs, constraints, proposed approach, pseudocode, complexity, and
   edge cases in docs/algorithms/. Then implement and validate the solution
   according to the documented design.
compatibility: opencode
---

# Algorithm First

## Purpose

This skill establishes a problem-solving workflow for algorithmic programming tasks.

The primary goal is to separate:

1. Problem understanding.
2. Algorithm design.
3. Implementation.
4. Validation.

The agent must not jump directly from the problem statement to source code.

The fundamental rule is:

> **Understand and design first. Implement second.**

This workflow is intended to support learning and understanding of algorithms, not merely obtaining working code.

---

## When to Use

Use this skill when the user asks to:

* Solve an algorithm exercise.
* Implement an algorithm.
* Solve a programming logic problem.
* Implement or analyze a data structure.
* Find an efficient algorithm.
* Optimize an existing algorithm.
* Compare algorithmic approaches.
* Analyze time or space complexity.
* Solve a problem involving arrays, lists, trees, graphs, sorting, searching, recursion, dynamic programming, or similar algorithmic techniques.

Do not use this workflow for trivial code changes that do not involve meaningful algorithmic reasoning, such as:

* Renaming a variable.
* Fixing a typo.
* Formatting code.
* Updating a dependency version.
* Changing a string or configuration value.
* Making a simple mechanical refactor.

---

# Mandatory Workflow

When this skill applies, follow these phases in order.

```text
Problem
   ↓
Analysis
   ↓
Algorithm design
   ↓
Documentation
   ↓
Implementation
   ↓
Testing
   ↓
Validation
```

Do not skip the documentation phase.

---

# Phase 1 — Understand the Problem

Before modifying source code, determine what the problem actually requires.

Identify:

* What must be solved.
* What information is provided.
* What result is expected.
* What constraints exist.
* What assumptions are valid.
* What cases may require special handling.

If the problem statement is ambiguous, ask the user for clarification before implementing.

Do not invent important requirements.

---

# Phase 2 — Define Input and Output

Determine the exact input and output of the algorithm.

Document:

* Input types.
* Input meaning.
* Input constraints.
* Output type.
* Output meaning.
* Special output conditions.

For example:

```text
Input:
An array of integers and a target integer.

Output:
The index of the target if it exists, otherwise -1.
```

---

# Phase 3 — Consider Possible Approaches

Before choosing an algorithm, consider reasonable alternatives.

For example:

```markdown
### Alternatives

1. Linear search
2. Binary search
```

Then compare them when relevant:

```markdown
| Approach | Time | Space | Requirement |
|----------|------|-------|-------------|
| Linear search | O(n) | O(1) | None |
| Binary search | O(log n) | O(1) | Sorted input |
```

Do not introduce alternatives merely for the sake of adding complexity.

Only discuss alternatives that are relevant to the problem.

---

# Phase 4 — Choose the Solution

Select the approach that best satisfies the problem's requirements.

Document why it was selected.

Example:

```markdown
### Selected approach

Binary search.

### Reason

The input array is guaranteed to be sorted, allowing the search
space to be divided in half on every iteration.
```

Correctness should take priority over premature optimization.

---

# Phase 5 — Create the Algorithm Document

Before implementing the solution, create a Markdown document inside:

```text
docs/algorithms/
```

If the directory does not exist, create it.

Use a descriptive filename.

Examples:

```text
docs/algorithms/001_linear_search.md
docs/algorithms/002_binary_search.md
docs/algorithms/003_two_sum.md
```

If the project already has an established naming convention, follow it.

The document must be created before modifying the implementation.

---

# Required Document Structure

The algorithm document must contain, when applicable:

````markdown
# <Problem name>

## Problem

Description of the problem.

## Input

Description of the input.

## Output

Description of the output.

## Constraints

Relevant constraints.

## Proposed Solution

Description of the chosen approach.

### Alternatives Considered

Relevant alternative approaches.

### Why This Approach

Reason for selecting the proposed approach.

## Algorithm

Step-by-step description.

### Pseudocode

Language-independent pseudocode.

## Complexity

### Time

O(...)

Explanation.

### Space

O(...)

Explanation.

## Edge Cases

- ...
- ...
- ...

## Examples

### Example 1

Input:

```text
...
````

Output:

```text
...
```

Explanation:

...

## Implementation

Files modified or created.

## Validation

Tests and verification performed.

## Observations

Additional notes.

````

Do not omit sections that are relevant to understanding the solution.

---

# Phase 6 — Write Pseudocode

The algorithm must be understandable independently of the programming language.

Use pseudocode before implementation.

Example:

```text
function binarySearch(array, target):
    left = 0
    right = length(array) - 1

    while left <= right:
        middle = floor((left + right) / 2)

        if array[middle] == target:
            return middle

        if array[middle] < target:
            left = middle + 1
        else:
            right = middle - 1

    return -1
````

Do not use language-specific syntax unless the syntax is necessary for understanding the algorithm.

---

# Phase 7 — Analyze Complexity

Document both:

```text
Time complexity
Space complexity
```

Use Big-O notation.

When useful, distinguish:

* Best case.
* Average case.
* Worst case.

Do not simply write `O(n)` without understanding why.

Example:

```markdown
### Time

O(log n)

Each iteration eliminates approximately half of the remaining
search space.

### Space

O(1)

Only a constant number of variables are used.
```

---

# Phase 8 — Identify Edge Cases

Consider cases that could cause incorrect behavior.

Examples:

* Empty input.
* Single-element input.
* Minimum valid input.
* Maximum valid input.
* Duplicate values.
* Missing values.
* Already sorted input.
* Reverse-sorted input.
* Negative numbers.
* Zero.
* Integer overflow, when applicable.
* Invalid input, when applicable.

Only include cases relevant to the specific problem.

---

# Phase 9 — Implement

Only after the algorithm document has been created and contains a sufficiently complete solution design may the agent modify source code.

The implementation must follow the documented algorithm.

Do not silently change the algorithm during implementation.

---

# If the Design Changes

Sometimes implementation reveals a problem with the original design.

If that happens:

1. Stop the implementation.
2. Identify what is wrong.
3. Update the algorithm document.
4. Explain the change in the document.
5. Continue implementation using the updated design.

The documentation must describe the algorithm that is actually implemented.

Never leave documentation describing an algorithm that differs materially from the implementation.

---

# Phase 10 — Validate

After implementation, verify the solution.

Check:

* Normal cases.
* Edge cases.
* Expected outputs.
* Constraints.
* Error conditions, when applicable.
* Time complexity.
* Space complexity.

If the project uses automated tests, create or update appropriate tests.

Prefer executing the tests instead of merely reasoning that they should pass.

---

# Phase 11 — Update the Algorithm Document

After implementation, update the document.

The implementation section should identify the relevant source files.

Example:

```markdown
## Implementation

Implemented in:

- `src/algorithms/binary_search.rs`
- `tests/binary_search.rs`
```

The validation section should record what was actually tested.

Example:

```markdown
## Validation

- [x] Normal case
- [x] Empty input
- [x] Single element
- [x] Element found
- [x] Element not found
- [x] Automated tests
```

Do not mark a test as completed unless it was actually verified.

---

# Educational Behavior

When the user is studying algorithms, prioritize understanding over simply producing code.

The explanation should make clear:

* Why the algorithm works.
* How the algorithm reaches the solution.
* Why the selected data structure is appropriate.
* Why one approach is better than another.
* How the algorithm behaves step by step.
* How its complexity is calculated.
* What happens in edge cases.

Do not unnecessarily hide the reasoning behind a finished implementation.

The user should be able to read the algorithm document without reading the source code and understand the intended solution.

---

# Do Not Over-Engineer

The purpose of this skill is algorithmic reasoning, not architectural complexity.

For a small algorithm exercise:

Do not introduce:

* Unnecessary abstractions.
* Complex project structures.
* Excessive design patterns.
* Unnecessary interfaces.
* Unrelated dependencies.

Keep the implementation proportional to the problem.

---

# Preserve the Learning History

Algorithm documents are part of the user's learning material.

Do not delete an existing problem document merely because the implementation has changed.

If a significant correction is made, document it under `Observations`.

Example:

```markdown
## Observations

The initial solution used O(n²) nested loops.

After analyzing the problem, it was replaced with a hash-based
approach with O(n) expected time complexity.
```

This allows the document to preserve the evolution of the solution.

---

# Existing Algorithm Documents

Before creating a new document, inspect `docs/algorithms/` if it exists.

Check whether:

* The same problem already exists.
* The project has an established naming convention.
* A previous solution can be reused or compared.
* The user is asking to improve an existing algorithm.

Do not create duplicate documents unnecessarily.

---

# Interaction With Existing Code

When the user asks to solve an algorithmic problem inside an existing project:

1. Inspect the relevant project structure.
2. Locate existing algorithm implementations.
3. Check existing tests.
4. Create or update the corresponding algorithm document.
5. Design the solution.
6. Implement it.
7. Run relevant tests.
8. Update the document with the result.

Do not modify unrelated parts of the project.

---

# Final Response

After completing the work, briefly report:

1. What problem was solved.
2. Where the algorithm document was created.
3. What algorithm was implemented.
4. Its time and space complexity.
5. What validation was performed.

Do not reproduce the entire algorithm document in the final response unless the user asks for it.

---

# Core Rule

When this skill is active, always remember:

> **Do not start by writing code. Start by understanding the problem, documenting the solution, and then implementing it.**
