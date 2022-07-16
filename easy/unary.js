const msg = readline();
debug = console.error;


const t = {'-':'','1':'0 ','0':'00 '}

let s = '';
let n = '-';
let m = 0;
for (let i in msg)
{
    let c = msg.charCodeAt(i).toString(2);
    c = '0'.repeat(7-c.length)+c;
    debug(c)
    for (let j in c)
    {
        const k = c[j];
        if (k!==n) 
        {
            s += t[n]+'0'.repeat(m)+' ';
            debug(n,k,"("+t[n]+")",m,s);
            n = k;
            m = 0;
        }
        m++;
    }
}
s += t[n]+'0'.repeat(m);
console.log(s.trim())

