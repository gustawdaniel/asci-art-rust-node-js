import readlineModule from "readline";

const rl = readlineModule.createInterface({
    input: process.stdin,
    output: process.stdout,
    terminal: false
});

const lines: string[] = [];
let readIndex = 0;

rl.on('line', (line: string) => {
    lines.push(line)
});

rl.on('close', () => {
    start();
})

function readline() {
    return lines[readIndex++];
}

class Alphabet {
    l: number = 0;
    h: number = 0;
    rows: string[] = []

    constructor(L: number, H: number) {
        this.l = L;
        this.h = H;
    }

    setRow(line: string) {
        this.rows.push(line.padEnd((25 + 2) * this.l, " "));
    }

    get(word: string): string[] {
        const aPosition = 'A'.charCodeAt(0);
        let rows = [...new Array(this.h)].map(() => '');
        // console.log("word", word);
        for (let letter of word) {
            let pos: number = letter.toUpperCase().charCodeAt(0) - aPosition;
            if (pos < 0 || pos > 25) pos = this.rows[0].length / this.l - 1;
            // console.log("pos", letter, pos, pos * (this.l), pos * (this.l + 1));
            for (let i = 0; i < this.h; i++) {
                rows[i] += this.rows[i].substring(pos * this.l, (pos + 1) * this.l);
            }
        }
        return rows;
    }
}

function start() {
    const L: number = parseInt(readline());
    const H: number = parseInt(readline());
    const T: string = readline();
    // console.log(L, H, T);
    const a = new Alphabet(L, H);
    for (let i = 0; i < H; i++) {
        const ROW: string = readline();
        a.setRow(ROW);
    }

    const res = a.get(T);
    res.forEach((row) => {
        console.log(row.replace(/\s+$/, ''));
    });
}
