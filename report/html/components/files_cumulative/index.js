import {line_chart} from "../../utils/echarts";
import {useEffect} from "preact/compat";

export default function FilesCumulative({data}){
    function rolling_sum_for_files(data){
        return data.reduce((pre, cur, index) => {
            let val = pre[index-1] === undefined ? 0 : pre[index-1][1];
            pre.push([cur[0],(cur[1]-cur[2])+val]);
            return pre;
        },[]);
    }

    useEffect(() => {
        line_chart("cumulative_files_by_day", rolling_sum_for_files(data));
    }, []);

    return (
        <div className="row">
            <div className="col">
                <h2>Cumulative Files</h2>
                <div id="cumulative_files_by_day" className="chart" />
            </div>
        </div>
    )
}