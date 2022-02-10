function restoreString (s, indices) {
    let map = new Map();
    {
        const str = s.split("")
        str.map((ele, index) => {
            map.set(indices[index], ele)
        })
        indices = null
    }
    let res = []
    const length = s.length
    for (let i = 0; i < length; i++){
        res.push(map.get(i))
    }
    return res.join("")
};