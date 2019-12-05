const fs = require('fs');
const sol1 = require('./solution1');


const walkTree = function(input){
    if(Math.floor(input)<=0){
        return 0
    }
    else{
        return walkTree(sol1.findMass(input))
    }
}

fs.readFile('questions/day1input.txt', 'utf8', (err, data)=>{
    const lines = data.split('\n');
    const modules = lines.map(v=>+(v));

    const moduleRequirements = modules.map(walkTree)

    console.log(moduleRequirements);



    // const fuelRequirements = modules.map(findMass);
    // const totalFuel = fuelRequirements.reduce((a,i)=>a+i);
    // console.log(`Total Fuel Requirements were: ${totalFuel}.`);
});


