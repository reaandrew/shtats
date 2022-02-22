import {h} from 'preact';
import * as echarts from "echarts";
import {useEffect} from "preact/compat";
import {bar_category} from "../../utils/echarts"

export default function CommitsByWeekday({data}) {
    function total_commits_by_weekday_model(){
        let obj = data.map(x=>[new Date(x[0]).getDay(),x[1]]).reduce((prev,curr) => {
            if (prev[curr[0]] === undefined){
                prev[curr[0]] = curr[1]
            }else{
                prev[curr[0]]+= curr[1]
            }
            return prev
        }, {0:0,1:0,2:0,3:0,4:0,5:0,6:0});
        return Object.keys(obj).map((key) => [Number(key), obj[key]])
    };

    useEffect(() => {
        let data = total_commits_by_weekday_model();
        let categories = ['Sun', 'Mon', 'Tue', 'Wed', 'Thu', 'Fri', 'Sat'];
        let values = data.map(x => x[1]);
        bar_category("total_commits_by_weekday", categories, values);

    }, []); //
    return (
        <div className="row">
            <div className="col">
                <h2>Commits by weekday</h2>
                <div id="total_commits_by_weekday" className="col chart"/>
            </div>
        </div>
    );
}