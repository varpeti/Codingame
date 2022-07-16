
const debug = console.error;

const arrToStr = (arr:Array<any>,sep:string=",\n\t",start:string="[\n\t",end:string="\n]\n") :string =>
{
    let str = start;
    for (let i in arr)
    {
        str += ""+arr[i]+sep;
    }
    str = str.slice(0, -sep.length);
    return str + end;
}

class Pos
{
    x:number;
    y:number;
    z:number;
    constructor(x:number,y:number,z:number)
    {
        this.x=x;
        this.y=y;
        this.z=z;
    }

    toString = () :string =>
    {
        return "("+this.x+","+this.y+","+this.z+")";
    }
}

class Box
{
    pos: Pos;
    size: Pos;

    constructor(pos:Pos,size:Pos)
    {
        this.pos = pos;
        this.size = size;
    }

    checkCollisionWith = (oth:Box) =>
    {
        const aPos = this.pos;
        const bPos = oth.pos;
        const aSize = this.size;
        const bSize = oth.size;
        //AABB (Axis Aligned Bounding Box)
        if (Math.abs(aPos.x - bPos.x) < aSize.x + bSize.x) {
            if (Math.abs(aPos.y - bPos.y) < aSize.y + bSize.y) {
                if (Math.abs(aPos.z - bPos.z) < aSize.z + bSize.z) {
                    return true;
                }
            }
        }
        return false;
    }

    toString = () :string =>
    {
        return "{pos:"+this.pos+",size:"+this.size+"}";
    }
}

class Apple
{
    pos:Pos;
    r:number;
    falling:boolean;
    box:Box;
    fallingBox:Box;

    /*
                +-+     110 +-+
                |x|  100 ^  | |
                +-+   ^  |  | |
                      |  |  | | 
                      |  |  |x|  55
                      |  |  | |  ^
                      |  |  | |  |
                      v  v  | |  v
        ---------0----------+-+----
    */

    constructor(pos:Pos,r:number)
    {
        this.pos = pos;
        this.r = r;
        this.falling = false;

        const height = (pos.z+r)/2;
        this.box = new Box(pos,new Pos(r,r,r));
        this.fallingBox = new Box(new Pos(pos.x,pos.y,height),new Pos(r,r,height));
    }

    toString = () :string =>
    {
        return "{pos:"+this.pos+",r:"+this.r+",falling:"+this.falling+",box:"+this.box+",fallingBox:"+this.fallingBox+"}";
    }

    fall = (apples:Apple[]) :Apple[] =>
    {
        this.falling = true;

        const hit : Apple[] = [];

        for (let i in apples)
        {
            const apple = apples[i];
            if (apple.falling) continue;
            if (this.fallingBox.checkCollisionWith(apple.box)) 
            {
                hit.push(apple);
            }
        }
        return hit;
    }
}

class Tree
{
    apples: Apple[] = [];

    constructor()
    {
        let inputs: string[] = readline().split(' ');
        const numberOfApples: number = parseInt(inputs[0]);
        const firstAppleToFall: number = parseInt(inputs[1]);
        for (let i = 0; i < numberOfApples; i++) 
        {
            let inputs: string[] = readline().split(' ');
            this.apples.push(
                new Apple(
                    new Pos(parseInt(inputs[0]),parseInt(inputs[1]),parseInt(inputs[2])),
                    parseInt(inputs[3])
                )
            );
        }
        this.solve(this.apples[firstAppleToFall])
    }


    solve = (firstAppleToFall:Apple) =>
    {
        let falling: Apple[] = [firstAppleToFall];
        while (falling.length>0)
        {
            const apple = falling.pop();
            falling = [...falling,...apple.fall(this.apples)];
        }

        let result = 0;
        for (let i in this.apples)
        {
            const apple = this.apples[i];
            if (!apple.falling) result++;
        }

        console.log(result);
    }

    toString = () :string =>
    {
        return arrToStr(this.apples);
    }
}

const tree = new Tree();
debug(""+tree);




