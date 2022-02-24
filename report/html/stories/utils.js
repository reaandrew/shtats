export function generate_data(days){
    const start_date = new Date('2021-01-01');
    let data = [...Array(days).keys()].map(days=>{
        return [start_date.addDays(days).toISOString().split('T')[0], Math.ceil(100000*Math.random()),Math.ceil(100000*Math.random())]
    })
    return data;
}