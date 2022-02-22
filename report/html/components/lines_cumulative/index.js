import {h} from 'preact';
import * as echarts from "echarts";
import {useEffect} from "preact/compat";
import {line_chart} from "../../utils/echarts";

export default function LinesCumulative({data}){
    function rolling_sum_for_lines(data){
        return data.reduce((pre, cur, index) => {
            let val = pre[index-1] === undefined ? 0 : pre[index-1][1];
            console.log('value', val);
            pre.push([cur[0],(cur[1]-cur[2])+val]);
            return pre;
        },[]);
    }

    useEffect(() => {
        line_chart("cumulative_lines_by_day", rolling_sum_for_lines(data));
    }, []);

    return (
        <div className="row">
            <div className="col">
                <h2>Cumulative Lines</h2>
                <div id="cumulative_lines_by_day" className="chart" />
            </div>
        </div>
    )
}