const fs = require('fs');
const solution1 = require('./solution1.js');

function walkTree(input){
    let fuel = solution1.findMass(input);
    let totalFuel = fuel;
    while(solution1.findMass(fuel)>=0){
        fuel = solution1.findMass(fuel);
        console.log(`Fuel is: ${fuel}`);
        totalFuel+=fuel
    }
    return totalFuel
}

modules = fs.readFileSync('questions/day1input.txt', 'utf8').split('\n').map(v=>+(v));

const sum = modules.map(walkTree).reduce((a,i)=>a+i);

console.log(`The sum is: ${sum}`);

