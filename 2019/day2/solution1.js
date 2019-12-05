const fs = require('fs');

const sampleCodes = fs.readFileSync('day2input.txt','utf8').split(',').map(v=>+v);


codes[1]=12;
codes[2]=2


function intCodes(codes){
    for(let i=0; i<=codes.length; i+=4){
        let instruction = codes[i];
        if(instruction===99){
            break;
        }
        else{
            const operands = [codes[codes[i+1]], codes[codes[i+2]]];
            let result;
            if(instruction===1){
                result = operands[0]+operands[1]
            }
            else if(instruction===2){
                result = operands[0]*operands[1]
            }
            codes[codes[i+3]]=result
        }

    }

    return codes;
}

console.log(`The first item in the array wound up as: ${intCodes(sampleCodes)[0]}`);