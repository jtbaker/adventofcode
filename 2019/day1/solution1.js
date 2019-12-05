const fs = require('fs');

const findMass = function(input){
    return Math.floor(input/3)-2
}

function q1p1(){
    fs.readFile('questions/day1input.txt', 'utf8', (err, data)=>{
        const lines = data.split('\n');
        const modules = lines.map(v=>+(v));
        const fuelRequirements = modules.map(findMass);
        const totalFuel = fuelRequirements.reduce((a,i)=>a+i);
        console.log(`Total Fuel Requirements were: ${totalFuel}.`);
    })  
}

// export default {
//     findMass: findMass
// }


module.exports = {
    findMass: findMass
}