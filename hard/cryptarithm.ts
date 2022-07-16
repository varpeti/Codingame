const debug = console.error;

const table = {};

class Word
{
    str: string = "";
    constructor(str:string)
    {
        this.str = str;
        for (let i=0; i<str.length; i++)
        {
            table[str[i]] = 0;
        }
    }

    getValue = (): number =>
    {
        if (table[this.str[0]] == 0) return NaN

        let value: number = 0;
        for (let i=0; i<this.str.length; i++)
        {
            value += table[this.str[i]]*Math.pow(10,this.str.length-i);
        }

        return value;
    }
}

class Cryptarithm
{
    words: Word[] = []; 
    total: Word;

    constructor(){}

    diff = ():number =>
    {
        let sum: number = 0;
        for (let i=0; i<this.words.length; i++)
        {
            sum += this.words[i].getValue();
        }
        return sum-this.total.getValue();
    }

    solve = () =>
    {
        const keys = Object.keys(table);
        keys.sort();
        for (let num=1; num<Math.pow(10,keys.length); num++)
        {
            const str = num.toString().padStart(keys.length,'0');

            const used = {};

            for (let i=0; i<keys.length; i++)
            {
                if (str[i] in used) break;
                table[keys[i]] = str[i];
                used[str[i]] = true;
            }

            const diff = this.diff();
            //debug(diff,table)
            if (diff===0) break;
        }

        for (let i=0; i<keys.length; i++)
        {
            console.log(keys[i],table[keys[i]]);
        }
    }
}

const cry = new Cryptarithm();

const N: number = parseInt(readline());
for (let i = 0; i < N; i++) 
{
    cry.words.push( new Word(readline()));
}
cry.total = new Word(readline());

debug(cry);

cry.solve();

