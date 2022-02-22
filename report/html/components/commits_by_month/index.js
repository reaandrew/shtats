import {h} from 'preact';
import * as echarts from "echarts";
import {bar_category} from "../../utils/echarts"
import {useEffect} from "preact/compat";

export default function CommitsByMonth({data}){
    const month_names_short=  ['Jan', 'Feb', 'Mar', 'Apr', 'May', 'Jun', 'Jul', 'Aug', 'Sep', 'Oct', 'Nov', 'Dec'];

    function total_commits_by_month_model(){
        let obj = data.map(x=>[new Date(x[0]).getMonth(),x[1]]).reduce((prev,curr) => {
            if (prev[curr[0]] === undefined){
                prev[curr[0]] = curr[1]
            }else{
                prev[curr[0]]+= curr[1]
            }
            return prev
        }, {});
        return Object.keys(obj).map((key) => [Number(key), obj[key]]);
    }

    useEffect(() => {
        let data = total_commits_by_month_model();
        let sorted_data = [...data].sort((a,b) => a[0]-b[1]);
        let categories = sorted_data.map(x => month_names_short[x[0]]);
        let values = sorted_data.map(x => x[1]);
        bar_category("total_commits_by_month", categories, values);
    }, []);

    return (
        <div className="row">
            <div className="col">
                <h2>Commits by month</h2>
                <div id="total_commits_by_month" className="col chart"/>
            </div>
        </div>
    );
}