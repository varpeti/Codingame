const debug = console.error;

class Instruction
{
    params: string[];
    constructor(line:string)
    {
        this.params = line.split(' ');
    }
}

class Interpreter
{
    registers = {a:0,b:0,c:0,d:0};
    instructions: Instruction[] = [];
    position: number = 0;

    constructor(input:string[])
    {
        this.registers["a"] = parseInt(input[0]);
        this.registers["b"] = parseInt(input[1]);
        this.registers["c"] = parseInt(input[2]);
        this.registers["d"] = parseInt(input[3]);
    }

    getParamValue = (str:string) : number =>
    {
        if (str in this.registers) return this.registers[str];
        return parseInt(str);
    }

    lookUpTable = 
    {
        "MOV": (dst:string,s1:string,_:null)=>
        {
            this.registers[dst] = this.getParamValue(s1);
        },
        "ADD": (dst:string,s1:string,s2:string)=>
        {
            this.registers[dst] = this.getParamValue(s1) + this.getParamValue(s2);
        },
        "SUB": (dst:string,s1:string,s2:string)=>
        {
            this.registers[dst] = this.getParamValue(s1) - this.getParamValue(s2);
        },
        "JNE": (imm:string,s1:string,s2:string)=>
        {
            if (this.getParamValue(s1) != this.getParamValue(s2)) this.position = parseInt(imm)-1;
        },
    }

    next = () =>
    {
        const i = this.instructions[this.position];
        this.lookUpTable[i.params[0]](i.params[1],i.params[2],i.params[3]);
        this.position++;
        //debug(interpreter);
    }

    run = () =>
    {
        while (this.position<this.instructions.length)
        {
            this.next()
        }
    }

    output = () =>
    {
        console.log(this.registers.a + " " + this.registers.b + " " + this.registers.c + " " + this.registers.d);
    }
}


let inputs: string[] = readline().split(' ');
const interpreter = new Interpreter(inputs);

const n: number = parseInt(readline());
for (let i = 0; i < n; i++) 
{
    interpreter.instructions.push( new Instruction(readline()) );
}

interpreter.run();

interpreter.output();

