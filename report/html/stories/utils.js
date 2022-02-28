export function generate_data(days){
    const start_date = new Date('2021-01-01');
    let data = [...Array(days).keys()].map(days=>{
        let lines_added = Math.ceil(100000*Math.random());
        let lines_deleted =  Math.ceil(lines_added * Math.random());
        return [start_date.addDays(days).toISOString().split('T')[0], lines_added,lines_deleted]
    })
    return data;
}