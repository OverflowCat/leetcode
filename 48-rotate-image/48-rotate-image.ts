/**
 Do not return anything, modify matrix in-place instead.
 */
function rotate(matrix: number[][]): void {
    const siz = matrix.length
    for (let i = 0; i < siz; i++) {
        for (let j = i; j < siz; j++) {
            const swap = matrix[i][j]
            matrix[i][j] = matrix[j][i]
            matrix[j][i] = swap
        }
    }
    for (let l = 0; l < siz; l++) {
        let i = 0, j = siz - 1
        while (i < j) {
            const swap = matrix[l][i]
            matrix[l][i] = matrix[l][j]
            matrix[l][j] = swap
            i++
            j--
        }
    }
}