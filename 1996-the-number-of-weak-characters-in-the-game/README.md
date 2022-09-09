<h2><a href="https://leetcode.com/problems/the-number-of-weak-characters-in-the-game/">1996. The Number of Weak Characters in the Game</a></h2><h3>Medium</h3><hr><div><p>You are playing a game that contains multiple characters, and each of the characters has <strong fr-fix-stroke="true">two</strong> main properties: <strong fr-fix-stroke="true">attack</strong> and <strong fr-fix-stroke="true">defense</strong>. You are given a 2D integer array <code>properties</code> where <code>properties[i] = [attack<sub>i</sub>, defense<sub>i</sub>]</code> represents the properties of the <code>i<sup>th</sup></code> character in the game.</p>

<p>A character is said to be <strong fr-fix-stroke="true">weak</strong> if any other character has <strong fr-fix-stroke="true">both</strong> attack and defense levels <strong fr-fix-stroke="true">strictly greater</strong> than this character's attack and defense levels. More formally, a character <code>i</code> is said to be <strong fr-fix-stroke="true">weak</strong> if there exists another character <code>j</code> where <code>attack<sub>j</sub> &gt; attack<sub>i</sub></code> and <code>defense<sub>j</sub> &gt; defense<sub>i</sub></code>.</p>

<p>Return <em>the number of <strong fr-fix-stroke="true">weak</strong> characters</em>.</p>

<p>&nbsp;</p>
<p><strong fr-fix-stroke="true">Example 1:</strong></p>

<pre><strong fr-fix-stroke="true">Input:</strong> properties = [[5,5],[6,3],[3,6]]
<strong fr-fix-stroke="true">Output:</strong> 0
<strong fr-fix-stroke="true">Explanation:</strong> No character has strictly greater attack and defense than the other.
</pre>

<p><strong fr-fix-stroke="true">Example 2:</strong></p>

<pre><strong fr-fix-stroke="true">Input:</strong> properties = [[2,2],[3,3]]
<strong fr-fix-stroke="true">Output:</strong> 1
<strong fr-fix-stroke="true">Explanation:</strong> The first character is weak because the second character has a strictly greater attack and defense.
</pre>

<p><strong fr-fix-stroke="true">Example 3:</strong></p>

<pre><strong fr-fix-stroke="true">Input:</strong> properties = [[1,5],[10,4],[4,3]]
<strong fr-fix-stroke="true">Output:</strong> 1
<strong fr-fix-stroke="true">Explanation:</strong> The third character is weak because the second character has a strictly greater attack and defense.
</pre>

<p>&nbsp;</p>
<p><strong fr-fix-stroke="true">Constraints:</strong></p>

<ul>
	<li><code>2 &lt;= properties.length &lt;= 10<sup>5</sup></code></li>
	<li><code>properties[i].length == 2</code></li>
	<li><code>1 &lt;= attack<sub>i</sub>, defense<sub>i</sub> &lt;= 10<sup>5</sup></code></li>
</ul>
</div>