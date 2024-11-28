# 浅谈生成函数 (Generating Function) - Elegia

本文原链接：[旧文补档 2018.7.31](https://www.luogu.com/article/atgvt10i)。

## 概念

本章讨论的生成函数指在某个环上的形式幂级数 (Formal Power Series) $R[[z]]$，也可表示为 $R^{\mathbb N}$。

## 运算

一个数列 $\langle a_0, a_1, \dots \rangle$ 的普通型生成函数 (Ordinary Generating Function) 为
$$
f(z) = \sum_{n=0}^{\infty} a_n z^n
$$
我们为了简化书写和保持形式的美感，一般情况下 $0^0 = 1$。

### 加法

首先我们可以导出加减运算的意义。

$\langle a_0 \pm b_0, a_1 \pm b_1, \dots \rangle$ 的生成函数为

$$
h(z) = f(z) \pm g(z)
$$

其系数表达可以在 $\Theta(n)$ 的时间内算出。

### 乘法

生成函数相乘对应数列的卷积运算。
$$
h(z) = f(z)g(z)
$$
$$
c_n = \sum_{i=0}^n a_ib_{n-i}
$$
其系数表达可以通过快速傅里叶变换在 $\Theta(n\log n)$ 的时间内算出。

特别地，乘法可以借助表示数列的移位，即 $g(z) = z^kf(z)$ 则我们有
$$
b_n = a_{n-k}
$$
除法通过乘法的逆元从而定义，所有常数项不为 0 的数列的生成函数都有对应的逆元。

其系数表达取同余 $z^n$ 的部分可以在 $\Theta(n\log n)$ 的时间内算出。

对于一些在数学中常见的函数，其泰勒级数 (Taylor Series) 是在形式演算中总是有意义的，即使在某些地方不收敛。由此我们可以找到一些特别的数列的生成函数。
$$
\begin{aligned}
\langle 1, 1, \dots \rangle &: & \frac 1{1-z} \\
\left\langle 1, \frac 1 {1!}, \frac 1 {2!}, \dots \right\rangle &: & {\mathrm{e}} ^z \\
\left\langle 0, \frac 1 1, -\frac 1 2, \frac 1 3, -\frac 1 4, \dots \right\rangle &: & \ln (1 + z)
\end{aligned}
$$
一些函数对应的数列也可以通过求导得到，即 $[z^n]f(z) = \frac 1{n!} f^{(n)}(0)$，但也有另一种借助 Cauchy 积分的方法（Cauchy's Residue Theorem）。
$$
[z^n]f(z) = \frac 1 {2\pi\mathrm{i}} \oint_{|z| = r} \frac{f(z){\,\mathrm{d}} z}{z^{n + 1}}
$$
（例：交错不等关系排列计数 $\Leftrightarrow$ $\tan$ 函数 $\Leftrightarrow$ Riemann $\zeta$ 函数）

函数的求导也可以对应多项式的求导。

$$
\begin{aligned}
\langle a_1, 2a_2, \dots \rangle &: & f'(z) \\
\left\langle 0, a_0, \frac 1 2 a_1, \dots \right\rangle &: & \int_0^z f(t){\,\mathrm{d}} t
\end{aligned}
$$

### Lagrange 反演

当一个函数满足 $z = f(w)$ 时，我们有其逆函数 $w = g(z)$ 的展开式即进行 Lagrange 反演 (Lagrange Inversion)
$$
[z^n]g(z) = \frac 1n[w^{n-1}]\left( \frac w{f(w)} \right)^n
$$
以及其扩展形式：
$$
[z^n]h(g(z)) = \frac1n [w^{n-1}] h'(w)\left( \frac w{f(w)} \right)^n
$$

### $k$ 叉树

因为 Lagrange 反演不允许常数项有值，所以我们的方程不允许单独的空树：
$$
T(z) = z(1 + T(z))^k
$$
我们有
$$
z = \frac{T(z)}{(1 + T(z))^k}
$$
即有逆函数
$$
f(w) = \frac{w}{(1 + w)^k}
$$
故有
$$
[z^n]T(z) = \frac 1n \binom{nk}{n - 1}
$$
只需根据组合数的性质稍作调整即可符合 $n = 0$ 的情况：
$$
\frac{\binom{nk}{n}}{n(k - 1) + 1}
$$

### 二项式

牛顿二项式是对于二项式定理的扩展，让指数可以不是非负整数。

$$
(1 + z)^\alpha = \sum_{n = 0}^{\infty} \binom{\alpha}{n} z^n
$$

这里的二项式系数是

$$
\binom{\alpha}{n} = \frac{\alpha (\alpha - 1) \cdots (\alpha - n + 1)}{n!}
$$

### 证明 Lucas 定理

即记 $\cdots n_2 n_1 n_0$ 为 $n$ 的 $p$ 进制表示，则
$$
\binom{n}{m} \equiv \binom{n_0}{m_0} \binom{n_1}{m_1} \cdots \pmod p
$$
首先通过 $p$ 是质数推导出
$$
0 < n < p \Rightarrow \left. p \middle \vert \binom{p}{n}\right.
$$
于是有

$$
(1+z)^p \bmod p = 1 + z^p
$$

进一步归纳得到

$$
(1+z)^{p^k} \bmod p = 1 + z^{p^k}
$$

即可得到结论。

#### 推导 Catalan 数列的通项公式

考虑 Catalan 数由组合意义导出的一个递推式

$$
\sum_{i = 0}^n C_iC_{n - i} = C_{n + 1}
$$

这个式子是卷积之后右移一位，因此可以得到 Catalan 数的 OGF

$$
c(z) = zc(z)^2 + 1
$$

其中的 $+1$ 是为了补上初始项。

接下来我们解出两个解

$$
c(z) = \frac{1 \pm \sqrt{1 - 4z}}{2z}
$$

但取正号时如果要求 $c(0)$ 则会发生非 0 数除以 0，这是必然不符合条件的，因此舍去正号，得到

$$
c(z) = \frac{1 - \sqrt{1 - 4z}}{2z}
$$

我们用牛顿二项式展开

$$
\begin{aligned}
\frac{1 - \sqrt{1 - 4z}}{2z} & = -\frac{\sum_{n = 1}^\infty \binom{\frac 1 2}{n}(-4z)^n}{2z} \\
& = \sum_{n = 0}^\infty 2\binom{\frac 1 2}{n + 1} (-4z)^n \\
& = \sum_{n = 0}^\infty 2\frac{\frac 1 2\left(\frac 1 2 - 1\right)\cdots \left(\frac 1 2 - n\right)}{(n + 1)!} (-4z)^n \\
& = \sum_{n = 0}^\infty \frac{\left(n - \frac 1 2\right)\cdots \left(2 -\frac 3 2\right) \left(1 - \frac 1 2\right)}{(n + 1)!} (4z)^n \\
& = \sum_{n = 0}^\infty \frac{(2n - 1)(2n - 3)\cdots 1}{(n + 1)!} (2z)^n \\
& = \sum_{n = 0}^\infty \frac{(2n)!}{n!(n + 1)!}z^n
\end{aligned}
$$

### 前缀和

如果希望将数列进行前缀和 $b_n = \sum_{i=0}^n a_i$，考虑这是一个卷积形式，即另一部分总是 1，那么可以得到

$$
g(z) = \frac 1{1 - z} f(z)
$$

相反地，差分函数则是 $1 - z$。

对一个数列做 $k$ 次前缀和，那么用于卷积的辅助函数 $\frac 1 {1 - z}$ 就被乘了 $k$ 次，由牛顿二项式可以得到

$$
\begin{aligned}
\left(\frac 1 {1 - z}\right)^k & = (1 - z)^{-k} \\
& = \sum_{n = 0}^\infty \binom{-k}{n} (-z)^n \\
& = \sum_{n = 0}^\infty \frac{(-k)(-k - 1)\cdots(-k - n + 1)}{n!} (-z)^n \\
& = \sum_{n = 0}^\infty \frac{(k + n - 1)\cdots(k + 1)k}{n!} z^n \\
& = \sum_{n = 0}^\infty \binom{n + k - 1}{k - 1} z^n
\end{aligned}
$$

### 变换

带入 $f(x^k)$ 可以将数列拉伸，如 $f(x^2)$ 可以得到 $\langle a_0, 0, a_1, 0, a_2, \dots \rangle$。

如果我们想得到 $\langle a_0, 0, a_2, 0, a_4, \dots \rangle$ 则可以根据奇函数和偶函数的性质构造

$$
g(z) = \frac{f(z) + f(-z)}{2}
$$

这利用到了 $1^n + (-1)^n$ 的周期性，如果我们想要得到

$$
b_n = \begin{cases}
a_n & \text{if}\quad n \equiv r \pmod m \\
0 & \text{otherwise}
\end{cases}
$$

则可以类比傅里叶变换得到

$$
g(z) = \frac 1 m \sum_{k = 0}^{m - 1} \omega_m^{-kr}f(\omega_m^k z)
$$

其中 $\omega$ 任取一个 $m$ 次主根，例如 $\omega = {\mathrm{e}}^{\frac{2\pi\mathrm{i}}m}$ 。

遗憾的是，这个主根在某些环上可能不存在。

#### 常系数递推

生成函数的方法可以帮助我们从另一种层面理解特征方程。

以斐波那契数列为例，由递推方程 $F_n = F_{n - 1} + F_{n - 2}$ 我们可以对应列出位移方法

$$
f(z) = z + zf(z) + z^2f(z)
$$

因此我们得到其生成函数

$$
f(z) = \frac z{1 - z - z^2}
$$

我们只需要解出 $1 - z - z^2 = 0$ 的根，根据裂项就可以将其化简成几个形如 $\frac 1{1 - qz}$ 的运算了。

那么根据一元二次方程的求根公式我们得到两根

$$
\phi = \frac{1 + \sqrt 5}2, \varphi = \frac{1 - \sqrt 5}2
$$

那么由根系关系我们知道了 $z^2 + z - 1 = (z - \phi)(z - \varphi)$

将原式裂开

$$
\begin{aligned}
f(z) & = \frac z{1 - z - z^2} \\
& = -\frac z{(z - \phi)(z - \varphi)} \\
& = \frac z{\phi - \varphi} \left(\frac 1{z - \phi} - \frac 1{z - \varphi}\right) \\
& = \frac z{\phi - \varphi} \sum_{n = 0}^\infty (\varphi^{-n - 1} - \phi^{-n - 1})z^n
\end{aligned}
$$

因此我们得到了

$$
F_n = \frac{\varphi^{-n} - \phi^{-n}}{\phi - \varphi}
$$

### 整数划分数列

记 $P_n$ 为 $n$ 可以无序拆分成若干的正整数的方案。

我们计数一个划分中用到了几个 $k$，那么可以写出生成函数

$$
p(z) = (1 + z + z^2 + \cdots)(1 + z^2 + z^4 + \cdots)(1 + z^3 + z^6 + \cdots)\cdots
$$

这可以化简为

$$
p(z) = \prod_{k = 1}^\infty \frac 1{1 - z^k}
$$

而 $\phi(z) = \prod_{k = 1}^\infty (1 - z^k)$ 的数列被称为五边形数，Euler 给出了其展开式

$$
\prod_{k = 1}^\infty (1 - z^k) = 1 + \sum_{k = 1}^\infty (-1)^k \left[z^{\frac{k(3k + 1)}2} + z^{\frac{k(3k - 1)}2}\right]
$$

其取值十分稀疏，直接求逆也可当做递推式，复杂度为 $\Theta(n\sqrt n)$，使用多项式牛顿迭代法求逆复杂度为 $\Theta(n\log n)$

### 下降幂

定义下降幂

$$
x^ {\underline n} = x(x - 1)\cdots(x - n + 1)
$$

下降幂和方幂可通过斯特林数系数转换

$$
x^{\underline n} = \sum_{k = 0}^n (-1)^{n - k}{\left[n \atop k\right]} x^k
$$

一般 $s(n, k) = (-1)^{n - k}{\left[n \atop k\right]}$ 被称为有标号第一类斯特林数。

而对于类似的上升幂则更简略，且可以通过组合意义的考量发现该式推出该式是正确的

$$
x^{\overline n} = x(x + 1)\cdots(x + n - 1) = \sum_{k = 0}^n {\left[n \atop k\right]} x^k
$$

事实上这几个结论可以通过几个简单的变换证明等价。

将方幂转为下降幂则是

$$
x^n = \sum_{k = 0}^n {n \brace k} x^{\underline k}
$$

转为上升幂则出现有标号第二类斯特林数

$$
x^n = \sum_{k = 0}^n (-1)^{n - k} {n \brace k} x^{\overline k}
$$

## 指数型生成函数

对于处理一类组合问题的时候，通常遇到的乘积可能是这样

$$
c_n = \sum_{i = 0}^n \binom{n}{i} a_ib_{n - i}
$$

经过变形

$$
\frac{c_n}{n!} = \sum_{i = 0}^n \frac{a_ib_{n - i}}{i!(n - i)!}
$$

发现在运算过程中适合如此定义其生成函数

$$
f(z) = \sum_{n = 0}^\infty a_n\frac{z^n}{n!}
$$

这种生成函数被称为数列的指数型生成函数 (Exponential Generating Function)

而这种情况下，数列 $\langle 1, 1, \dots \rangle$ 的生成函数是 ${\mathrm{e}}^z$，相应地有些运算律也有变化。接下来展示一个简单的例子体现如何使用指数型生成函数来解决一个组合问题。

在一条长为 $n$ 的方格上涂色，有红黄蓝三种，其中红色必须出现一次，黄色必须出现偶数次，蓝色最多出现一次。
z
这三个不同颜色独立出现的方案的 EGF 分别是 ${\mathrm{e}}^z - 1, \frac{{\mathrm{e}}^z + {\mathrm{e}}^{-z}}2, 1 + z$。因此整体方案数就是三种颜色组合相乘

$$
({\mathrm{e}}^z - 1) \cdot \frac{{\mathrm{e}}^z + {\mathrm{e}}^{-z}}2 \cdot (1 + z)
$$

注意到化简的式子形如 ${\mathrm{e}}^{qz}$ 对应的数列是 $\langle q^n \rangle$，而 $z{\mathrm{e}}^{qz}$ 对应的是 $\langle nq^{n - 1} \rangle$

化简后可得方案为

$$
a_n = \frac 12 \left(2^n - 1 + [n = 0] - (-1)^n + n2^{n - 1} - n + [n = 1] + n(-1)^n \right)
$$

### 集合划分

$\mathrm{Bell}_n$ 数列是 $n$ 个元素的无序集合划分的数量，而第二类 Stirling 数 ${n \brace k}$ 是 $n$ 个元素被划分至 $k$ 个非空集合的方案数量，通过 EGF 我们可以轻松得到它们的生成函数。

首先考虑这 $n$ 个元素被分成了多少个集合，而每个集合非空则 EGF 为 ${\mathrm{e}}^z - 1$，如果分成了 $m$ 个集合，那么我们可以得到生成函数

$$
\sum_{n = 0}^\infty {n \brace k} \frac{z^n}{n!} = \frac{({\mathrm{e}}^z - 1)^k}{k!}
$$

这意味着我们给每个元素染上了一个颜色的同时保证每个颜色至少染了一个元素。最后除以 $k!$ 则对颜色的序去重。

对这个式子应用二项式定理展开

$$
\frac 1{k!} \sum_{j = 0}^k \binom{k}{j} (-1)^{k - j} {\mathrm{e}}^{jz}
$$

之后得到的就是我们熟悉的容斥法计算第二类 Stirling 数

$$
{n \brace k} = \frac 1{k!}\sum_{j = 0}^k (-1)^{k - j} \binom{k}{j} j^n
$$

而对于 Bell 数来说，不限制集合数量，所以是对于所有不同 $k$ 的第二类 Stirling 数的生成函数之和

$$
\begin{aligned}
B(z) & = \sum_{k = 0}^\infty \frac{({\mathrm{e}}^z - 1)^k}{k!} \\
& = {\mathrm{e}}^{{\mathrm{e}}^z - 1}
\end{aligned}
$$

由多项式牛顿迭代，通过多项式 $\exp$ 可以在 $\Theta(n\log n)$ 的时间内算出 $1 \sim n$ 内的所有 Bell 数。

### 第一类 Stirling 数

类比集合，圆排列的特点在于第一个位置是灵活的，因此单个圆排列数列应当是 $\langle (n-1)! \rangle$，其 EGF 为

$$
\sum_{n = 1}^\infty (n - 1)! \frac{z^n}{n!} = \sum_{n = 1}^\infty \frac {z^n}n
$$

系数为倒数令我们想起对数函数 $\ln(1 + z) = \sum_{n = 1}^\infty \frac 1 n (-1)^{n - 1}z^n$，所以可以得到我们想要的函数为

$$
-\ln(1 - z) = \ln \frac 1{1 - z}
$$

$$
\sum_{n = 0}^\infty {\left[n \atop k\right]} \frac{z^n}{n!} = \frac{\left(\ln \frac 1{1-z}\right)^k}{k!}
$$

### 二项式反演 (Binomial Inversion)

当两个数列满足关系

$$
b_n = \sum_{i = 0}^n \binom{n}{i} a_i
$$

时，我们称这是一个二项式变换 (Binomial Transform)，可以用二项式定理证明反演

$$
a_n = \sum_{i = 0}^n (-1)^{n - i}\binom n i b_i
$$

但究其原因，我们发现两个数列的 EGF 由前者等式得到的是

$$
g(z) = {\mathrm{e}}^{z} f(z)
$$

因此必然满足

$$
f(z) = {\mathrm{e}}^{-z} g(z)
$$

其实更加优美的表达形式是

$$
b_n = \sum_{i = 0}^n (-1)^i\binom{n}{i} a_i
$$

### Stirling 反演

注意当两个数列满足关系

$$
b_n = \sum_{k = 0}^n {n \brace k} a_k
$$

时，我们称其为 Stirling 变换，其 EGF 对应的有

$$
g(z) = f({\mathrm{e}}^z - 1)
$$

因此我们换元可以得到

$$
f(z) = g(\ln (1 + z))
$$

故得到 Stirling 反演公式

$$
a_n = \sum_{k = 0}^n (-1)^{n - k} {n \brack k} b_k
$$

### Codeforces 961G Partitions

依据题意可直观推出一个式子

$$
\sum_{i = 1}^n i\binom{n-1}{i-1}{n-i \brace k-1}
$$

但是对于此题来说 $n \le 2\times 10^5$，不能有效地进行计算。我们考虑式子已经具有卷积的可能，对其进行初等变形

$$
\begin{aligned}
a_n & = \sum_{i = 1}^n i\binom{n-1}{i-1}{n-i \brace k-1} \\
& = \sum_{i = 0}^n \frac{i^2}n\binom{n}{i}{n - i\brace k - 1} \\
& = \frac{b_n}n \\
b_n & = \sum_{i = 0}^n \left[i + i(i - 1) \right]\binom{n}{i}{n - i\brace k - 1} \\
B(z) & = (z{\mathrm{e}}^z + z^2{\mathrm{e}}^z)\frac{({\mathrm{e}}^z - 1)^{k - 1}}{(k - 1)!} \\
B(z) & = (z + z^2)\left[ k \frac{({\mathrm{e}}^z - 1)^k}{k!} + \frac{({\mathrm{e}}^z - 1)^{k - 1}}{(k - 1)!} \right] \\
b_n & = kn\left[{n-1\brace k}+(n-1){n-2\brace k}\right] + n\left[{n-1\brace k-1}+(n-1){n-2\brace k-1}\right]\\
& = n\left(k{n-1\brace k}+{n-1\brace k-1}\right) + n(n - 1)\left(k{n-2\brace k}+{n-2\brace k - 1}\right) \\
& = n{n\brace k} + n(n-1){n - 1\brace k} \\
a_n & = {n\brace k}+(n-1){n-1\brace k}
\end{aligned}
$$

## 概率与期望

生成函数可以辅助研究一类特殊的概率问题。

记发生事件所带来的效果为 $n$ 的概率为 $p_n$，那么它的 OGF 即

$$
f(z) = \sum_{n = 0}^\infty p_nz^n
$$

有几个相关的意义

$$
f(1) = \sum_{n = 0}^\infty p_n = 1
$$

这一点必须满足。

$$
f'(1) = \sum_{n = 0}^\infty np_n = \operatorname{mean}(f)
$$

代表这一事件的期望。

而方差是

$$
\begin{aligned}
\operatorname{var}(f) &= \sum_{n = 0}^\infty p_n(n - \operatorname{mean}(f)) ^ 2 \\
&= \sum_{n = 0}^\infty n^2p_n - \operatorname{mean}(f)^2 \\
&= f''(1) + f'(1) - f'(1)^2
\end{aligned}
$$

## 解析组合 (Analytic Combinatorics)

解析组合试图从一个较为机械化的方式帮助我们将组合计数问题从模型直接转为生成函数。

### 组合类

一个组合类 (Combinatorial Class) $\mathcal A$ 是一个有限集或可数集，即 $|\mathcal A| \le \aleph_0$，并且集合中的每一个元素 $\alpha$ 我们赋予一个属性 $|\alpha|$ 为这个元素的“大小”。但我们要求

- 每个元素的大小是一个非负整数。
- 对于每个非负整数 $n$，以其为大小的元素是有限的。

因此我们可以定义有限集 $\mathcal A_n$ 为所有大小为 $n$ 的元素的集合，而 $A_n = |\mathcal A_n|$。

因此，在我们理解数列的普通型生成函数的时候，可以如此理解

$$
A(z) = \sum_{\alpha \in \mathcal A} z^{|\alpha|}
$$

形参 $z$ 刻画了元素的大小。

### 运算

当 $\Phi$ 将 $m$ 个组合类联系成一个组合类

$$
\mathcal A = \Phi \left[\mathcal B^{(1)}, \mathcal B^{(2)}, \dots, \mathcal B^{(m)}\right]
$$

且数列 $A_n$ 只与 $B^{(1)}_n, B^{(2)}_n, \dots, B^{(m)}_n$ 数列有关时，我们感兴趣的是转换方式即映射 $\Phi$ 对应的运算 $\Psi$

$$
A(z) = \Psi \left[B^{(1)}(z), B^{(2)}(z), \dots, B^{(m)}(z)\right]
$$

#### 笛卡尔积

当一个类 $\mathcal A$ 由笛卡尔积 (Cartesian Product) 的方式生成的时候

$$
\mathcal A \cong \mathcal B \times \mathcal C = \{ \alpha = (\beta, \gamma) | \beta \in \mathcal B \wedge \gamma \in \mathcal C \}
$$

在一般情况下新元素的意义是

$$
|\alpha| = |\beta| + |\gamma|
$$

因此可以得到生成函数对应的是

$$
A(z) = B(z)C(z)
$$

#### 和

当一个类 $\mathcal A$ 由两个类 $\mathcal B, \mathcal C$ 合并生成时（默认类与类之间没有共同部分，即 $\mathcal B \cap \mathcal C = \emptyset$）

我们干脆称这个运算为 $\mathcal A = \mathcal B + \mathcal C$ 两个类的组合和 (Sum)

我们有

$$
A(z) = B(z) + C(z)
$$

#### Catalan 数

以二叉树的方式理解 Catalan 类 $\mathcal C$，我们可以得到一个构造方式：

一个元素要么为空树，要么分为自己节点，左子树和右子树。因此我们可以列出组合类的自指构造过程

$$
\mathcal C = \{\epsilon\} + \mathcal C \times \mathcal Z \times \mathcal C
$$

默认记 $\mathcal Z$ 是一个单一元素类，只包含一个 $|z| = 1$。

因此生成函数必然是

$$
C(z) = 1 + C(z)zC(z)
$$

#### 序列构造

一个组合类 $\mathcal A$ 组成的不定长序列 (Sequence) 为

$$
\operatorname{SEQ}(\mathcal A) = \{ \epsilon \} + \mathcal A + \mathcal A \times \mathcal A + \cdots
$$

对应的生成函数

$$
\frac1{1 - A(z)}
$$

记 $$Q[f] = \frac1{1 - f}$$

被称为 $f$ 的准逆元 (Quasi Inversion)

如果我们想要进行对有根树计数，且儿子有顺序则有

$$
\mathcal T = \mathcal Z \times \operatorname{SEQ}(\mathcal T)
$$

对应的 OGF 满足

$$
T(z) = \frac{z}{1 - T(z)}
$$

可以解得

$$
T(z) = \frac{1 - \sqrt{1- 4z}}2 = \sum_{n = 1}^\infty \frac{{2n - 2 \choose n - 1}}n z^n
$$

#### 幂集

一个类的幂集 (Power Set) 即其集合的所有子集，枚举每个元素是否存在可以得到

$$
\operatorname{PSET}(\mathcal A) = \prod_{\alpha \in \mathcal A} (\{\epsilon\} + \{\alpha\})
$$

因此生成函数为

$$
\prod_{\alpha \in \mathcal A} (1 + z^{|\alpha|}) = \prod_{n = 0}^\infty (1 + z^n)^{A_n}
$$

另一种不含多项式系数的表示方法可以对式子先取 $\ln$ 再取 $\exp$

$$
\begin{aligned}
\prod_{n = 0}^\infty (1 + z^n)^{A_n}
& = \exp\left( \sum_{n = 1}^\infty A_n \ln (1 + z^n)\right) \\
& = \exp\left( \sum_{n = 1}^\infty A_n \sum_{k = 1}^\infty \frac{(-1)^{k - 1}}{k} z^{nk}\right) \\
& = \exp\left( \sum_{k = 1}^\infty \frac{(-1)^{k - 1}}{k} \sum_{n = 1}^\infty A_n \cdot (z^k)^n \right) \\
& = \exp\left( \sum_{k = 1}^\infty \frac{(-1)^{k - 1}}k A(z^k) \right)
\end{aligned}
$$

记

$$
\operatorname{\overline {Exp}} [f] = \exp\left( \sum_{k = 1}^\infty \frac{(-1)^{k - 1}}k f(z^k) \right)
$$

被称为改版 P\'olya 指数 (Modified P\'olya Exponential)

#### 多重集

一个类的多重集 (Multiset) 则每个元素自成序列

$$
\operatorname{MSET}(\mathcal A) = \prod_{\alpha \in \mathcal A} \operatorname{SEQ}(\{\alpha\})
$$

因此生成函数是

$$
\prod_{n = 1}^\infty (1 - z^n)^{-A_n}
$$

类似可以证明它等于

$$
\exp\left(\sum_{k = 1}^\infty \frac{A(z^k)}{k}\right)
$$

记

$$
\operatorname{Exp} [f] = \exp\left(\sum_{k = 1}^\infty \frac{f(z^k)}{k}\right)
$$

这一运算被称为 P\'olya 指数 (P\'olya Exponential)

#### 圆排列

圆排列 (Cycle) 或者说环 $\operatorname{CYC}(\mathcal A)$ 是对序列以平移操作进行去重。

注意到由 P\'olya 定理，若环由 $n$ 个元素组成则对旋转变换群 $R_n$ 去重

$$
A_n(z) = \frac 1 n\sum_{d = 1}^n A\left(z^{\frac{n}{\gcd(n, d)}} \right)^{\gcd(n, d)}
$$

考虑到对 $\gcd$ 的结果统计

$$
A_n(z) = \frac1n \sum_{k = 1}^n \varphi(k) A(z^k)^{n/k}
$$

$$
\begin{aligned}
\sum_{n = 1}^\infty A_n(z) & = \sum_{n = 1}^\infty \frac1n \sum_{k = 1}^n \varphi(k) A(z^k)^{n/k} \\
& = \sum_{k = 1}^\infty \varphi(k) \sum_{m = 1}^\infty \frac 1{mk} A(z^k)^m\\
& = \sum_{k = 1}^\infty \frac{\varphi(k)}k \ln \frac1{1 - A(z^k)}
\end{aligned}
$$

记

$$
\operatorname{Log} [f] = \sum_{k = 1}^\infty \frac{\varphi(k)}k \ln \frac1{1 - f(z^k)}
$$

被称为 P\'olya 对数 (P\'olya Logarithm)

### 拆分

一般来说，拆分是对于一个正整数集 $\mathcal I = \mathcal Z + \mathcal Z^2 + \cdots$ 的子集 $\mathcal T \subseteq \mathcal I$

#### 有序切分 (Composition)

将一个数用 $\mathcal T$ 中的数有序拆分，记为 $\mathcal C^{\mathcal T}$

其实容易得到

$$
\mathcal C^{\mathcal T} = \operatorname{SEQ}(\mathcal T)
$$

故

$$
C^{\mathcal T}(z) = \frac1{1-T(z)}
$$

而注意到 Fibonacci 数便可以理解为 $\mathcal T = \{1, 2\}$ 的拆分。

#### 无序拆分 (Partition)

记为 $\mathcal P^{\mathcal T}$

$$
\mathcal P^{\mathcal T} = \operatorname{MSET}(\mathcal T)
$$

有

$$
P^{\mathcal T}(z) = \prod_{n \in \mathcal T}\frac1{1-z^n}
$$

这类数一般难有通项公式，但有近似

$$
P^{(\le r)}_n \sim \frac{n^{r-1}}{r!(r-1)!}
$$

通过组合思维可以推导出一个不平凡的式子

$$
\prod_{n = 1}^\infty \frac 1{(1-z^n)} = \sum_{h = 0}^\infty \frac{z^{h^2}}{\prod_{k = 1}^h (1-z^k)^2}
$$

这是考虑将一个拆分排成一个图形，分割中间的中心正方形得到的。

所以我们得到了

$$
\mathcal P^{\mathcal I} = \bigcup_{h = 0}^\infty \mathcal P^{(\le h)} \mathcal Z^{h^2} \mathcal P^{(\le h)}
$$

还有一个有趣的应用可以证明：相异无序拆分等于奇无序拆分。

记 $\mathcal Q, \mathcal O$ 为互异数拆分方案和奇数拆分方案。

$$
\begin{aligned}
Q(z) & = \prod_{n = 1}^\infty (1 + z^n) \\
O(z) & = \frac1{1-z} \frac1{1-z^3} \frac1{1-z^5} \cdots
\end{aligned}
$$

只要发现 $1 + z^n = \frac{1-z^{2n}}{1-z^n}$，因此

$$
\begin{aligned}
Q(z) & = \frac{1-z^2}{1-z} \frac{1-z^4}{1-z^2} \frac{1-z^6}{1-z^3} \cdots \\
& = \frac1{1-z} \frac1{1-z^3} \frac1{1-z^5} \cdots \\
& = O(z)
\end{aligned}
$$

#### 环切分 (Cyclic Composition)

环切分的一般情况较难分析，记 $\mathcal D = \operatorname{CYC}(\mathcal I)$，我们有

$$
D(z) = \sum_{k = 1}^\infty \frac{\varphi(k)}{k}\ln \frac1{1 - \frac{z^k}{1-z^k}}
$$

暴力分析系数可以得到通项公式

$$
\begin{aligned}
D_n &= \frac1n \sum_{k|n} \varphi(k)(2^\frac n k - 1) \\
& = -1 + \frac1n \sum_{k|n} \varphi(k) 2^\frac n k
\end{aligned}
$$

### 最长频次

记一个单词 $\mathcal W$ 类中只有字符 $a, b$ 考虑其中 $a$ 的连续子段长度小于 $k$ ，被称为一个 Longest Run 的计数。

由

$$
\mathcal W = a_{<k} \operatorname{SEQ}(b a_{<k})
$$

$$
\begin{aligned}
W^{\langle k\rangle}(z) & = \frac{1-z^k}{1-z} \cdot \frac1{1 - z\frac{1-z^k}{1-z}} \\
& = \frac{1-z^k}{1-2z-z^{k+1}}
\end{aligned}
$$

推广地，如果字符集大小为 $m$ 则为

$$
\frac{1-z^k}{1-mz-(m-1)z^{k+1}}
$$

如果对 $a, b$ 均有限制 (Double Run)，则考虑将构造分解为

$$
\mathcal W^{\langle \alpha,\beta \rangle} = \operatorname{SEQ}_{\le \alpha}(a)\operatorname{SEQ}(b\operatorname{SEQ}_{<\beta}(b)a\operatorname{SEQ}_{<\alpha}(a))\operatorname{SEQ}_{\le\beta}(b)
$$

因此有

$$
W^{\langle \alpha,\beta \rangle}(z) = \frac{(1-z^{\alpha+1})(1-z^{\beta+1})}{(1-z)^2-z^2(1-z^\alpha)(1-z^\beta)}
$$

### 自动机识别

对于一个语言 $\mathcal L$，有字符集 $\mathcal A$，可行的字符串当且仅当可以被给定的确定性有限状态自动机 (DFA, Deterministic Finite-state Automaton) 识别。其中状态为 $Q = \{q_0, q_1, \dots, q_s\}$，而被识别是到达终止节点 $\overline Q$ 中的某一个。

$$
L(z) = \mathbf u(I - zT)^{-1}\mathbf v^{\mathsf T}
$$

其中 $u_i = 1$ 代表 $q_i$ 是初始状态，$v_i = 1$ 代表 $q_i \in \overline Q$，而 $T_{i, j}$ 表示有多少种字符可以使 $i$ 到 $j$。

可以分状态讨论证明，设 $\mathcal L_i$ 是所有从 $q_i$ 出发的情况。

$$
\mathcal L_i = \Delta_i + \sum_{\alpha \in \mathcal A} \{\alpha\}\mathcal L_{q_i \cdot \alpha}
$$

当 $q_i \in \overline Q$ 时，$\Delta_i = \{\epsilon\}$，否则为 $\emptyset$。

事实上解这个方程就能得到这一结论。

例如匹配串 $abb$，可以解得生成函数

$$
L(z) = \frac{z^3}{(1-z)(1-2z)(1-z-z^2)} = \frac1{1-2z} -\frac{2+z}{1-z-z^2} + \frac1{1-z}
$$

得到通项公式

$$
L_n = 2^n - F_{n + 3} + 1
$$

## 狄利克雷级数

狄利克雷级数 (Dirichlet Series) 的特点在于其级数的乘法对应的卷积是狄利克雷卷积。是指数级数，也可以认为是卷积为狄利克雷卷积的数列狄利克雷生成函数 (Dirichlet Generating Function)。

$$
\mathfrak D_w^A(s) = \sum_{\alpha \in A} \frac1{w(\alpha)^s} = \sum_{n=1}^\infty \frac {a_n}{n^s}
$$

发现在这个级数中，$\langle 1, 1 \cdots \rangle$ 对应的就是 Riemann Zeta 函数 $\zeta(s) = \sum_{n=1}^\infty \frac 1{n^s}$。可以预见狄利克雷级数的演算可能用到大量复分析的知识，因此不太可能引入到实际比赛的题目中，但可以作为狄利克雷卷积更深刻的了解。

### 质因子分解

对于所有的积性数列来说，其狄利克雷级数可以分解成每个质因子指数基的级数乘积，不同的 $p^k$ 是正交的。

我们可以由此推导出一个式子

$$
\begin{aligned}
\sum_{n=1}^\infty \frac1{n^s} & = \sum_{p_1^{k_1}p_2^{k_2}\cdots} \frac 1{p_1^{sk_1}p_2^{sk_2}\cdots} \\
& = \left(\sum_{k_1 = 0}^\infty \frac1{p_1^{sk_1}}\right)\left(\sum_{k_2 = 0}^\infty \frac1{p_2^{sk_2}}\right)\cdots \\
& = \prod_{p \in \mathbb P} \frac1{1 - p^{-s}}
\end{aligned}
$$

### Mobius 变换与反演

事实上所有的 Mobius 变换在级数上的体现就是将一个函数乘以 $\zeta(s)$，不难验证

$$
\frac 1{\zeta(s)} = \sum_{n=1}^\infty \frac{\mu(n)}{n^s} = \prod_{p \in \mathbb P}(1 - p^{-s})
$$

所以我们常用与反演的 M\&quot;obius 函数 $\mu(n)$ 当做数列，其级数就是 $\zeta(s)$ 的倒数函数。

#### Euler 函数

由欧拉函数满足恒等式

$$
\sum_{d|n} \varphi(d) = n
$$

而

$$
\sum_{n = 1}^\infty \frac{n}{n^s} = \sum_{n = 1}^\infty \frac1{n^{s - 1}} = \zeta(s - 1)
$$

可以得到

$$
\sum_{n = 1}^\infty \frac{\varphi(n)}{n^s} = \frac{\zeta(s - 1)}{\zeta(s)}
$$

### 除数函数

$$
\sum_{n = 1}^\infty \frac{\sigma_k(n)}{n^s} = \zeta(s - k)\zeta(s)
$$

### 求导

根据一些简单的求导知识我们可以得到

$$
f'(s) = \sum_{n = 1}^\infty \frac{-a_n\ln n}{n^s}
$$

因此我们有

$$
-\frac{\zeta'(s)}{\zeta(s)} = \sum_{n = 1}^\infty \frac{\Lambda(n)}{n^s}
$$

我们得到了 von Mangoldt 函数 $\Lambda(n)$
