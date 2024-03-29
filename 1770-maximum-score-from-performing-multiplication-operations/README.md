<h2><a href="https://leetcode.com/problems/maximum-score-from-performing-multiplication-operations/">1770. Maximum Score from Performing Multiplication Operations</a></h2><h3>Medium</h3><hr><div><p>You are given two integer arrays <code>nums</code> and <code>multipliers</code><strong fr-fix-stroke="true"> </strong>of size <code>n</code> and <code>m</code> respectively, where <code>n &gt;= m</code>. The arrays are <strong fr-fix-stroke="true">1-indexed</strong>.</p>

<p>You begin with a score of <code>0</code>. You want to perform <strong fr-fix-stroke="true">exactly</strong> <code>m</code> operations. On the <code>i<sup>th</sup></code> operation <strong fr-fix-stroke="true">(1-indexed)</strong>, you will:</p>

<ul>
	<li>Choose one integer <code>x</code> from <strong fr-fix-stroke="true">either the start or the end </strong>of the array <code>nums</code>.</li>
	<li>Add <code>multipliers[i] * x</code> to your score.</li>
	<li>Remove <code>x</code> from the array <code>nums</code>.</li>
</ul>

<p>Return <em>the <strong fr-fix-stroke="true">maximum</strong> score after performing </em><code>m</code> <em>operations.</em></p>

<p>&nbsp;</p>
<p><strong fr-fix-stroke="true">Example 1:</strong></p>

<pre><strong fr-fix-stroke="true">Input:</strong> nums = [1,2,3], multipliers = [3,2,1]
<strong fr-fix-stroke="true">Output:</strong> 14
<strong fr-fix-stroke="true">Explanation:</strong>&nbsp;An optimal solution is as follows:
- Choose from the end, [1,2,<strong fr-fix-stroke="true"><u fr-fix-stroke="true">3</u></strong>], adding 3 * 3 = 9 to the score.
- Choose from the end, [1,<strong fr-fix-stroke="true"><u fr-fix-stroke="true">2</u></strong>], adding 2 * 2 = 4 to the score.
- Choose from the end, [<strong fr-fix-stroke="true"><u fr-fix-stroke="true">1</u></strong>], adding 1 * 1 = 1 to the score.
The total score is 9 + 4 + 1 = 14.</pre>

<p><strong fr-fix-stroke="true">Example 2:</strong></p>

<pre><strong fr-fix-stroke="true">Input:</strong> nums = [-5,-3,-3,-2,7,1], multipliers = [-10,-5,3,4,6]
<strong fr-fix-stroke="true">Output:</strong> 102
<strong fr-fix-stroke="true">Explanation: </strong>An optimal solution is as follows:
- Choose from the start, [<u><strong fr-fix-stroke="true">-5</strong></u>,-3,-3,-2,7,1], adding -5 * -10 = 50 to the score.
- Choose from the start, [<strong fr-fix-stroke="true"><u fr-fix-stroke="true">-3</u></strong>,-3,-2,7,1], adding -3 * -5 = 15 to the score.
- Choose from the start, [<strong fr-fix-stroke="true"><u fr-fix-stroke="true">-3</u></strong>,-2,7,1], adding -3 * 3 = -9 to the score.
- Choose from the end, [-2,7,<strong fr-fix-stroke="true"><u fr-fix-stroke="true">1</u></strong>], adding 1 * 4 = 4 to the score.
- Choose from the end, [-2,<strong fr-fix-stroke="true"><u fr-fix-stroke="true">7</u></strong>], adding 7 * 6 = 42 to the score. 
The total score is 50 + 15 - 9 + 4 + 42 = 102.
</pre>

<p>&nbsp;</p>
<p><strong fr-fix-stroke="true">Constraints:</strong></p>

<ul>
	<li><code>n == nums.length</code></li>
	<li><code>m == multipliers.length</code></li>
	<li><code>1 &lt;= m &lt;= 10<sup>3</sup></code></li>
	<li><code>m &lt;= n &lt;= 10<sup>5</sup></code><code> </code></li>
	<li><code>-1000 &lt;= nums[i], multipliers[i] &lt;= 1000</code></li>
</ul>
</div>