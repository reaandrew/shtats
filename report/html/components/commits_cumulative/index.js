import {h} from 'preact';
import * as echarts from "echarts";
import {useEffect} from "preact/compat";
import {line_chart} from "../../utils/echarts"


export default function CommitsCumulative({data}){
    function rolling_sum(data){
        return data.reduce((pre, cur, index) => {
            let val = pre[index-1] === undefined ? 0 : pre[index-1][1];
            pre.push([cur[0],cur[1]+val]);
            return pre;
        },[]);
    }

    useEffect(() => {
        line_chart("cumulative_commits_by_day", rolling_sum(data))
    }, []);

    return (
        <div className="row">
            <div className="col">
                <h2>Cumulative commits</h2>
                <div id="cumulative_commits_by_day" className="col chart" />
            </div>
        </div>
    )
}