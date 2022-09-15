<h2><a href="https://leetcode.com/problems/find-original-array-from-doubled-array/">2007. Find Original Array From Doubled Array</a></h2><h3>Medium</h3><hr><div><p>An integer array <code>original</code> is transformed into a <strong fr-fix-stroke="true">doubled</strong> array <code>changed</code> by appending <strong fr-fix-stroke="true">twice the value</strong> of every element in <code>original</code>, and then randomly <strong fr-fix-stroke="true">shuffling</strong> the resulting array.</p>

<p>Given an array <code>changed</code>, return <code>original</code><em> if </em><code>changed</code><em> is a <strong fr-fix-stroke="true">doubled</strong> array. If </em><code>changed</code><em> is not a <strong fr-fix-stroke="true">doubled</strong> array, return an empty array. The elements in</em> <code>original</code> <em>may be returned in <strong fr-fix-stroke="true">any</strong> order</em>.</p>

<p>&nbsp;</p>
<p><strong fr-fix-stroke="true">Example 1:</strong></p>

<pre><strong fr-fix-stroke="true">Input:</strong> changed = [1,3,4,2,6,8]
<strong fr-fix-stroke="true">Output:</strong> [1,3,4]
<strong fr-fix-stroke="true">Explanation:</strong> One possible original array could be [1,3,4]:
- Twice the value of 1 is 1 * 2 = 2.
- Twice the value of 3 is 3 * 2 = 6.
- Twice the value of 4 is 4 * 2 = 8.
Other original arrays could be [4,3,1] or [3,1,4].
</pre>

<p><strong fr-fix-stroke="true">Example 2:</strong></p>

<pre><strong fr-fix-stroke="true">Input:</strong> changed = [6,3,0,1]
<strong fr-fix-stroke="true">Output:</strong> []
<strong fr-fix-stroke="true">Explanation:</strong> changed is not a doubled array.
</pre>

<p><strong fr-fix-stroke="true">Example 3:</strong></p>

<pre><strong fr-fix-stroke="true">Input:</strong> changed = [1]
<strong fr-fix-stroke="true">Output:</strong> []
<strong fr-fix-stroke="true">Explanation:</strong> changed is not a doubled array.
</pre>

<p>&nbsp;</p>
<p><strong fr-fix-stroke="true">Constraints:</strong></p>

<ul>
	<li><code>1 &lt;= changed.length &lt;= 10<sup>5</sup></code></li>
	<li><code>0 &lt;= changed[i] &lt;= 10<sup>5</sup></code></li>
</ul>
</div>