<h2><a href="https://leetcode.com/problems/utf-8-validation/">393. UTF-8 Validation</a></h2><h3>Medium</h3><hr><div><p>Given an integer array <code>data</code> representing the data, return whether it is a valid <strong fr-fix-stroke="true">UTF-8</strong> encoding (i.e. it translates to a sequence of valid UTF-8 encoded characters).</p>

<p>A character in <strong fr-fix-stroke="true">UTF8</strong> can be from <strong fr-fix-stroke="true">1 to 4 bytes</strong> long, subjected to the following rules:</p>

<ol>
	<li>For a <strong fr-fix-stroke="true">1-byte</strong> character, the first bit is a <code>0</code>, followed by its Unicode code.</li>
	<li>For an <strong fr-fix-stroke="true">n-bytes</strong> character, the first <code>n</code> bits are all one's, the <code>n + 1</code> bit is <code>0</code>, followed by <code>n - 1</code> bytes with the most significant <code>2</code> bits being <code>10</code>.</li>
</ol>

<p>This is how the UTF-8 encoding would work:</p>

<pre>     Number of Bytes   |        UTF-8 Octet Sequence
                       |              (binary)
   --------------------+-----------------------------------------
            1          |   0xxxxxxx
            2          |   110xxxxx 10xxxxxx
            3          |   1110xxxx 10xxxxxx 10xxxxxx
            4          |   11110xxx 10xxxxxx 10xxxxxx 10xxxxxx
</pre>

<p><code>x</code> denotes a bit in the binary form of a byte that may be either <code>0</code> or <code>1</code>.</p>

<p><strong fr-fix-stroke="true">Note: </strong>The input is an array of integers. Only the <strong fr-fix-stroke="true">least significant 8 bits</strong> of each integer is used to store the data. This means each integer represents only 1 byte of data.</p>

<p>&nbsp;</p>
<p><strong fr-fix-stroke="true">Example 1:</strong></p>

<pre><strong fr-fix-stroke="true">Input:</strong> data = [197,130,1]
<strong fr-fix-stroke="true">Output:</strong> true
<strong fr-fix-stroke="true">Explanation:</strong> data represents the octet sequence: 11000101 10000010 00000001.
It is a valid utf-8 encoding for a 2-bytes character followed by a 1-byte character.
</pre>

<p><strong fr-fix-stroke="true">Example 2:</strong></p>

<pre><strong fr-fix-stroke="true">Input:</strong> data = [235,140,4]
<strong fr-fix-stroke="true">Output:</strong> false
<strong fr-fix-stroke="true">Explanation:</strong> data represented the octet sequence: 11101011 10001100 00000100.
The first 3 bits are all one's and the 4th bit is 0 means it is a 3-bytes character.
The next byte is a continuation byte which starts with 10 and that's correct.
But the second continuation byte does not start with 10, so it is invalid.
</pre>

<p>&nbsp;</p>
<p><strong fr-fix-stroke="true">Constraints:</strong></p>

<ul>
	<li><code>1 &lt;= data.length &lt;= 2 * 10<sup>4</sup></code></li>
	<li><code>0 &lt;= data[i] &lt;= 255</code></li>
</ul>
</div>