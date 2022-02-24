import {h} from 'preact';
import * as echarts from "echarts";
import FilesCumulative from "../files_cumulative";
import FilesSummary from "../files_summary";


export default function Lines({data}) {
    return (
        <div>
            <div className="row">
                <div className="col">
                    <FilesCumulative data={data}/>
                </div>
                <div className="col-3">
                    <FilesSummary data={data} />
                </div>
            </div>
        </div>
    )
}