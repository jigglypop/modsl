# MoDsl

[![Rust](https://github.com/jigglypop/modsl/actions/workflows/rust.yml/badge.svg)](https://github.com/jigglypop/modsl/actions/workflows/rust.yml)

**MoDsl**은 "five times the sum of two and three"와 같이, 사람이 사용하는 자연어를 컴퓨터가 이해할 수 있는 수학적 표현으로 변환하는 Rust 기반의 DSL(Domain-Specific Language)입니다.

수학 기호에 익숙하지 않은 사용자나, 더 직관적인 방식으로 수식을 다루고 싶은 모두를 위해 만들어졌습니다.

---

## ✨ 주요 기능 (Features)

*   **직관적인 자연어 입력**: `plus`, `times`와 같은 일상적인 단어를 사용하여 수식을 작성할 수 있습니다.
*   **기호와 혼용 가능**: `5 * (3 plus 4)`처럼 수학 기호와 자연어를 함께 사용할 수 있어 유연성이 높습니다.
*   **연산자 우선순위**: 일반적인 수학 규칙에 따라 `*`, `/`가 `+`, `-`보다 먼저 계산되며, `( )`를 통해 연산 순서를 지정할 수 있습니다.
*   **Rust 기반**: Rust의 안정성과 성능을 바탕으로 빠르고 정확하게 수식을 파싱합니다.

## 🚀 빠른 시작 (Quick Start)

1.  **저장소 복제:**
    ```bash
    git clone https://github.com/your-username/modsl.git
    cd modsl
    ```

2.  **프로젝트 실행:**
    컴퓨터에 [Rust 및 Cargo](https://www.rust-lang.org/tools/install)가 설치되어 있어야 합니다.
    ```bash
    cargo run
    ```

    **실행 결과:**
    ```
    MoDsl - A new era of mathematical notation.

    Parsing: 5 times (3 plus 4) - 1

    Tokens: [Number(5.0), Op(Multiply), LParen, Number(3.0), Op(Add), Number(4.0), RParen, Op(Subtract), Number(1.0)]

    Parsed AST: BinaryOp(BinaryOp(Number(5.0), Multiply, BinaryOp(Number(3.0), Add, Number(4.0))), Subtract, Number(1.0))
    ```

## 🗺️ 로드맵 (Roadmap)

MoDsl은 이제 막 첫걸음을 떼었으며, 아래와 같은 방향으로 발전해 나갈 계획입니다.

*   **Phase 1: 핵심 DSL 및 계산기 구현**
    *   [x] 기본 사칙연산 파서
    *   [ ] AST(추상 구문 트리) 계산 기능 (`eval` 함수)
    *   [ ] 변수 및 대수 연산 지원
    *   [ ] 안정적인 오류 처리 시스템

*   **Phase 2: 고급 수학 기능 확장**
    *   [ ] 미적분 (`derivative`, `integral`) 및 선형대수(`matrix`, `vector`) 구문 추가
    *   [ ] `sin`, `cos`, `log` 등 기본 함수 지원
    *   [ ] `sum`, `product` 등 시그마, 파이 표현 지원

*   **Phase 3: 생태계 확장**
    *   [ ] Web API 및 CLI 인터페이스 제공
    *   [ ] LaTeX, MathML 등 다양한 포맷으로 출력하는 백엔드 구현
    *   [ ] Jupyter Notebook 연동 지원

*   **Phase 4: AI 통합 및 다국어 지원**
    *   [ ] LLM을 활용한 더욱 유연한 자연어 이해
    *   [ ] 한국어 등 다국어 지원 확장

## 💡 핵심 컨셉 (Core Concepts)

### 자연어의 모호성 해결

"the sum of 4 and 5 times 6"과 같이 모호한 표현을 해석하기 위해, MoDsl은 명확한 규칙을 적용하고 향후 사용자에게 의도를 확인하는 메커니즘을 도입할 것입니다.

### 확장 가능한 아키텍처

`Lexer` -> `Parser` -> `AST` -> `Backend`로 이어지는 파이프라인 구조를 통해, 새로운 문법을 추가하거나 다른 출력 포맷(예: 시각화, 코드 생성)을 지원하기 용이하도록 설계합니다.

## 🤝 기여하기 (Contributing)

MoDsl은 이제 막 시작하는 오픈소스 프로젝트입니다. 버그 리포트, 기능 제안, 코드 기여 등 어떤 형태의 참여든 환영합니다.

기여에 관심이 있으시다면 `Issues` 탭을 확인하거나 새로운 `Pull Request`를 생성해주세요.

## 📜 라이선스 (License)

이 프로젝트는 [MIT License](LICENSE)에 따라 배포됩니다.

## ✨ 현재 상태 및 지원 기능 (Current Status & Features)

현재 MoDsl은 초기 개발 단계에 있으며, 기본적인 사칙연산을 기호와 자연어로 혼용하여 파싱하는 기능을 지원합니다.

*   **기본 연산자:**
    *   덧셈: `+`, `plus`
    *   뺄셈: `-`, `minus`
    *   곱셈: `*`, `times`
    *   나눗셈: `/`, `divided by`
*   **그룹화:**
    *   괄호 `( ... )`를 사용한 연산자 우선순위 지정
*   **입력 형식:**
    *   기호와 자연어를 자유롭게 섞어서 사용 가능 (예: `5 * (3 plus 4)`)

## 🌟 핵심 가치와 비전 (Core Value & Vision)

### 1. **접근성 혁신**
- **진입 장벽 완화**: "integral from 0 to infinity of e to the minus x squared dx" 같은 자연스러운 영어 표현으로 수식 입력 가능
- **교육적 가치**: 수학 기호에 익숙하지 않은 학습자들도 복잡한 수식을 쉽게 표현
- **범용성**: 초등 산술부터 고등 미적분, 선형대수까지 통합된 인터페이스

### 2. **기술적 우수성**
- **모듈화 설계**: Parser, AST, Backend를 명확히 분리한 아키텍처
- **다중 백엔드 지원**: SymPy(심볼릭 연산)와 PyTorch(수치 연산)를 상황에 맞게 선택
- **다양한 출력 형식**: LaTeX, MathML, Unicode 등 용도별 포맷 지원

### 3. **실용적 확장성**
- CLI, Web API, Jupyter 통합 등 다양한 인터페이스
- PyPI 배포를 통한 쉬운 설치와 활용
- 체계적인 테스트 구조

## 잠재적 문제점과 개선 방향

### 1. **자연어의 모호성**
```
문제: "the sum of 4 and 5 times 6"
해석1: (4 + 5) × 6 = 54
해석2: 4 + (5 × 6) = 34
```
**개선안**: 
- 컨텍스트 기반 우선순위 규칙 명확화
- 모호한 경우 사용자에게 명확화 요청
- 괄호 사용 권장 메시지

### 2. **다국어 지원의 복잡성**
현재 영어 중심 설계로 한국어 같은 다른 언어 지원 시 문법 구조 재설계 필요
```
영어: "derivative of sin x with respect to x"
한국어: "sin x를 x에 대해 미분"
```
**개선안**: 
- 언어별 문법 모듈 분리
- 다국어 키워드 매핑 테이블
- 언어 감지 및 자동 전환

### 3. **성능 최적화**
- 복잡한 수식의 파싱 오버헤드
- SymPy의 심볼릭 연산 속도 한계
- 대규모 행렬 연산 시 메모리 관리

**개선안**:
- 자주 사용되는 표현 캐싱
- 병렬 처리 가능한 연산 식별
- Progressive parsing (점진적 파싱)

### 4. **오류 처리와 사용자 경험**
```python
# 현재 접근법의 한계
"solve x^2 + 2x + 5 = 0 for x"  # 복소수 해
"integral of 1/x from -1 to 1"   # 발산 적분
```
**개선안**:
- 수학적 예외 상황 감지 및 설명
- 대안 제시 (예: "복소수 해가 있습니다")
- 단계별 풀이 과정 표시 옵션

## 혁신적 확장 아이디어

### 1. **AI 통합**
- LLM을 활용한 더 자유로운 자연어 해석
- 수식 의도 파악 및 수정 제안
- 문제 해결 과정 설명 생성

### 2. **시각화 엔진**
```python
# 예시 확장
"plot derivative of sin x from 0 to 2pi"
"visualize eigenvalues of matrix A"
```

### 3. **교육 플랫폼 통합**
- 단계별 풀이 과정 생성
- 유사 문제 자동 생성
- 학습 진도 추적

### 4. **협업 기능**
- 수식 공유 및 버전 관리
- 실시간 공동 편집
- 수식 라이브러리 구축

## 시장성 분석

**타겟 사용자**:
1. 수학 교육자와 학생
2. 데이터 과학자 (빠른 프로토타이핑)
3. 연구자 (논문 작성 시 수식 검증)
4. 엔지니어 (계산 검증)

**경쟁 우위**:
- Wolfram Alpha보다 자연스러운 입력
- Mathematica보다 가벼운 설치
- 오픈소스로 커스터마이징 가능

## 구현 우선순위 제안

1. **Phase 1**: 핵심 산술/대수 DSL + CLI
2. **Phase 2**: 미적분/선형대수 확장 + Web API
3. **Phase 3**: 시각화 + 교육 기능
4. **Phase 4**: AI 통합 + 다국어 지원



# 발음 가능한 모든 수식의 영어 표현 케이스

## 1. 기본 산술 연산 (Basic Arithmetic)

### 사칙연산
- `3 + 5` → "three plus five"
- `7 - 2` → "seven minus two"
- `4 × 6` → "four times six" / "four multiplied by six"
- `8 ÷ 2` → "eight divided by two" / "eight over two"
- `-5` → "negative five" / "minus five"
- `+3` → "positive three" / "plus three"

### 거듭제곱과 근
- `2^3` → "two to the power of three" / "two cubed" / "two to the third"
- `x^n` → "x to the power of n" / "x to the n" / "x to the nth power"
- `√9` → "square root of nine" / "the square root of nine"
- `∛8` → "cube root of eight" / "the cube root of eight"
- `⁴√16` → "fourth root of sixteen" / "the fourth root of sixteen"
- `ⁿ√x` → "nth root of x" / "the nth root of x"

### 분수와 비율
- `3/4` → "three over four" / "three fourths" / "three divided by four"
- `a/b` → "a over b" / "a divided by b"
- `22/7` → "twenty-two over seven"
- `5:3` → "five to three" / "the ratio of five to three"
- `2.5` → "two point five" / "two and a half"

## 2. 대수학 표현 (Algebraic Expressions)

### 변수와 계수
- `2x` → "two x" / "two times x"
- `3xy` → "three x y" / "three times x times y"
- `x²` → "x squared" / "x to the second power"
- `x³` → "x cubed" / "x to the third power"
- `ax + b` → "a x plus b" / "a times x plus b"

### 방정식과 부등식
- `x = 5` → "x equals five" / "x is equal to five"
- `x ≠ 3` → "x is not equal to three" / "x does not equal three"
- `x < 7` → "x is less than seven"
- `x > 2` → "x is greater than two"
- `x ≤ 9` → "x is less than or equal to nine"
- `x ≥ 4` → "x is greater than or equal to four"
- `2 < x < 5` → "x is between two and five" / "two is less than x is less than five"

### 절대값과 괄호
- `|x|` → "absolute value of x" / "the absolute value of x"
- `|x - 3|` → "absolute value of x minus three"
- `(x + 2)` → "x plus two in parentheses" / "open parenthesis x plus two close parenthesis"
- `[x]` → "x in brackets" / "open bracket x close bracket"
- `{x}` → "x in braces" / "open brace x close brace"

## 3. 함수와 관계 (Functions and Relations)

### 기본 함수
- `f(x)` → "f of x" / "function f of x"
- `g(x, y)` → "g of x and y" / "function g of x comma y"
- `sin x` → "sine x" / "sine of x"
- `cos θ` → "cosine theta" / "cos theta"
- `tan α` → "tangent alpha" / "tan alpha"
- `log x` → "log x" / "logarithm of x"
- `ln x` → "natural log of x" / "l n of x"
- `e^x` → "e to the x" / "e to the power of x"

### 역함수와 합성
- `f⁻¹(x)` → "f inverse of x" / "the inverse function of f of x"
- `sin⁻¹ x` → "inverse sine of x" / "arc sine of x"
- `f ∘ g` → "f composed with g" / "f circle g"
- `(f ∘ g)(x)` → "f of g of x"

## 4. 미적분학 (Calculus)

### 극한
- `lim_{x→a} f(x)` → "the limit as x approaches a of f of x"
- `lim_{x→∞} f(x)` → "the limit as x approaches infinity of f of x"
- `lim_{x→0⁺} f(x)` → "the limit as x approaches zero from the right of f of x"
- `lim_{x→0⁻} f(x)` → "the limit as x approaches zero from the left of f of x"

### 미분
- `df/dx` → "d f d x" / "the derivative of f with respect to x"
- `∂f/∂x` → "partial f partial x" / "the partial derivative of f with respect to x"
- `f'(x)` → "f prime of x" / "the derivative of f of x"
- `f''(x)` → "f double prime of x" / "the second derivative of f of x"
- `d²y/dx²` → "d squared y d x squared" / "the second derivative of y with respect to x"
- `∇f` → "gradient of f" / "del f" / "nabla f"

### 적분
- `∫ f(x) dx` → "integral of f of x d x" / "the integral of f of x with respect to x"
- `∫ₐᵇ f(x) dx` → "integral from a to b of f of x d x" / "the definite integral from a to b of f of x d x"
- `∮ F·dr` → "the line integral of F dot d r" / "the closed line integral of F dot d r"
- `∬ f dA` → "double integral of f d A" / "the double integral of f over the area"
- `∭ f dV` → "triple integral of f d V" / "the triple integral of f over the volume"

## 5. 선형대수 (Linear Algebra)

### 벡터
- `v⃗` → "vector v" / "v vector"
- `a⃗ + b⃗` → "vector a plus vector b"
- `a⃗ · b⃗` → "a dot b" / "the dot product of a and b"
- `a⃗ × b⃗` → "a cross b" / "the cross product of a and b"
- `|v⃗|` → "magnitude of v" / "the magnitude of vector v"
- `‖v‖` → "norm of v" / "the norm of v"

### 행렬
- `A^T` → "A transpose" / "the transpose of A"
- `A⁻¹` → "A inverse" / "the inverse of A"
- `det(A)` → "determinant of A" / "the determinant of A"
- `tr(A)` → "trace of A" / "the trace of A"
- `A × B` → "A times B" / "the matrix product of A and B"
- `[aᵢⱼ]` → "the matrix with entries a i j"

## 6. 집합론과 논리 (Set Theory and Logic)

### 집합 연산
- `A ∪ B` → "A union B" / "the union of A and B"
- `A ∩ B` → "A intersect B" / "A intersection B" / "the intersection of A and B"
- `A \ B` → "A minus B" / "A set minus B" / "the set difference of A and B"
- `A ⊆ B` → "A is a subset of B" / "A is contained in B"
- `A ⊂ B` → "A is a proper subset of B"
- `x ∈ A` → "x is in A" / "x is an element of A" / "x belongs to A"
- `x ∉ A` → "x is not in A" / "x is not an element of A"

### 논리 연산
- `p ∧ q` → "p and q" / "p logical and q"
- `p ∨ q` → "p or q" / "p logical or q"
- `¬p` → "not p" / "negation of p"
- `p → q` → "p implies q" / "if p then q"
- `p ↔ q` → "p if and only if q" / "p iff q"
- `∀x` → "for all x" / "for every x"
- `∃x` → "there exists x" / "for some x"

## 7. 확률과 통계 (Probability and Statistics)

### 확률
- `P(A)` → "P of A" / "the probability of A"
- `P(A|B)` → "P of A given B" / "the probability of A given B"
- `P(A ∩ B)` → "P of A intersect B" / "the probability of A and B"
- `E[X]` → "E of X" / "the expected value of X" / "the expectation of X"
- `Var(X)` → "variance of X" / "the variance of X"
- `σ` → "sigma" / "standard deviation"

### 조합론
- `n!` → "n factorial"
- `C(n,k)` → "n choose k" / "the binomial coefficient n choose k"
- `P(n,k)` → "n permute k" / "the number of permutations of n taken k at a time"
- `(n k)` → "n choose k" / "binomial n k"

## 8. 수론 (Number Theory)

- `a ≡ b (mod n)` → "a is congruent to b modulo n" / "a is congruent to b mod n"
- `gcd(a,b)` → "g c d of a and b" / "the greatest common divisor of a and b"
- `lcm(a,b)` → "l c m of a and b" / "the least common multiple of a and b"
- `a | b` → "a divides b" / "a is a divisor of b"
- `p ∤ n` → "p does not divide n"

## 9. 특수 상수와 기호 (Special Constants and Symbols)

### 그리스 문자
- `α` → "alpha"
- `β` → "beta"
- `γ` → "gamma"
- `Δ` → "delta" / "capital delta"
- `ε` → "epsilon"
- `θ` → "theta"
- `λ` → "lambda"
- `μ` → "mu"
- `π` → "pi"
- `Σ` → "sigma" / "capital sigma" / "sum"
- `Π` → "pi" / "capital pi" / "product"
- `φ` → "phi"
- `ω` → "omega"

### 특수 기호
- `∞` → "infinity"
- `≈` → "approximately equal to" / "is approximately"
- `∝` → "is proportional to" / "proportional to"
- `±` → "plus or minus" / "plus minus"
- `∓` → "minus or plus" / "minus plus"
- `⊥` → "perpendicular to" / "is perpendicular to"
- `∥` → "parallel to" / "is parallel to"
- `∴` → "therefore"
- `∵` → "because" / "since"

## 10. 급수와 곱 (Series and Products)

- `Σᵢ₌₁ⁿ aᵢ` → "sum from i equals one to n of a i" / "the sum of a i from i equals one to n"
- `Σₙ₌₀^∞ xⁿ` → "sum from n equals zero to infinity of x to the n"
- `Πᵢ₌₁ⁿ aᵢ` → "product from i equals one to n of a i"

## 11. 복소수 (Complex Numbers)

- `i` → "i" / "the imaginary unit"
- `3 + 4i` → "three plus four i"
- `z̄` → "z bar" / "z conjugate" / "the complex conjugate of z"
- `Re(z)` → "real part of z" / "the real part of z"
- `Im(z)` → "imaginary part of z" / "the imaginary part of z"
- `|z|` → "modulus of z" / "the modulus of z" / "absolute value of z"

## 사용 예시 (Complete Examples)

1. `∫₀^∞ e^(-x²) dx = √π/2`
   → "integral from zero to infinity of e to the negative x squared d x equals square root of pi over two"

2. `lim_{x→0} (sin x)/x = 1`
   → "the limit as x approaches zero of sine x over x equals one"

3. `∀ε>0, ∃δ>0 : |x-a|<δ ⇒ |f(x)-L|<ε`
   → "for all epsilon greater than zero, there exists delta greater than zero such that absolute value of x minus a less than delta implies absolute value of f of x minus L less than epsilon"

4. `det(A - λI) = 0`
   → "determinant of A minus lambda I equals zero"

이러한 표현들을 DSL 문법에 포함시키면 대부분의 수학적 표현을 자연스럽게 입력할 수 있습니다.

# 마크다운에서의 수식 렌더링 가능 여부

## 렌더링 지원 플랫폼별 현황

### ✅ **완전 지원** (LaTeX 수식 자동 렌더링)
- **GitHub**: 2022년부터 `$` 및 `$$` 지원
- **GitLab**: MathJax 내장
- **Jupyter Notebook**: 기본 지원
- **Obsidian**: 기본 지원
- **Notion**: 인라인/블록 수식 지원
- **VS Code**: Markdown Preview Enhanced 확장

### ⚠️ **부분 지원** (추가 설정 필요)
- **Jekyll**: MathJax/KaTeX 플러그인 필요
- **Hugo**: 테마별 설정 필요
- **Gatsby**: 플러그인 설치 필요

### ❌ **미지원**
- **기본 Markdown 파서**: 수식 문법 없음
- **Reddit**: LaTeX 미지원
- **Discord**: 수식 렌더링 없음

## 실제 렌더링 예시

### 인라인 수식 (Inline Math)
```markdown
이차방정식 $ax^2 + bx + c = 0$의 해는 $x = \frac{-b \pm \sqrt{b^2-4ac}}{2a}$입니다.
```

### 블록 수식 (Display Math)
```markdown
$$
\int_0^{\infty} e^{-x^2} dx = \frac{\sqrt{\pi}}{2}
$$
```
