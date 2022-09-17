<h2><a href="https://leetcode.com/problems/palindrome-pairs/">336. Palindrome Pairs</a></h2><h3>Hard</h3><hr><div><p>Given a list of <b fr-fix-stroke="true">unique</b> words, return all the pairs of the&nbsp;<b fr-fix-stroke="true"><i fr-fix-stroke="true">distinct</i></b> indices <code>(i, j)</code> in the given list, so that the concatenation of the two words&nbsp;<code>words[i] + words[j]</code> is a palindrome.</p>

<p>&nbsp;</p>
<p><strong fr-fix-stroke="true">Example 1:</strong></p>

<pre><strong fr-fix-stroke="true">Input:</strong> words = ["abcd","dcba","lls","s","sssll"]
<strong fr-fix-stroke="true">Output:</strong> [[0,1],[1,0],[3,2],[2,4]]
<strong fr-fix-stroke="true">Explanation:</strong> The palindromes are ["dcbaabcd","abcddcba","slls","llssssll"]
</pre>

<p><strong fr-fix-stroke="true">Example 2:</strong></p>

<pre><strong fr-fix-stroke="true">Input:</strong> words = ["bat","tab","cat"]
<strong fr-fix-stroke="true">Output:</strong> [[0,1],[1,0]]
<strong fr-fix-stroke="true">Explanation:</strong> The palindromes are ["battab","tabbat"]
</pre>

<p><strong fr-fix-stroke="true">Example 3:</strong></p>

<pre><strong fr-fix-stroke="true">Input:</strong> words = ["a",""]
<strong fr-fix-stroke="true">Output:</strong> [[0,1],[1,0]]
</pre>

<p>&nbsp;</p>
<p><strong fr-fix-stroke="true">Constraints:</strong></p>

<ul>
	<li><code>1 &lt;= words.length &lt;= 5000</code></li>
	<li><code>0 &lt;= words[i].length &lt;= 300</code></li>
	<li><code>words[i]</code> consists of lower-case English letters.</li>
</ul>
</div>