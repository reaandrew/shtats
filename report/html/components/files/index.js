import {h} from 'preact';
import * as echarts from "echarts";
import FilesCumulative from "../files_cumulative";
import FilesSummary from "../files_summary";
import FilesByExtension from "../files_by_extension";

export default function Lines({data, file_extension_data}) {
    return (
        <div>
            <div className="row">
                <div className="col">
                    <FilesCumulative data={data}/>
                </div>
            </div>
            <div className="row">
                <div className="col-6">
                    <FilesByExtension data={file_extension_data} />
                </div>
                <div className="col-6">
                    <FilesSummary data={data} />
                </div>
            </div>

        </div>
    )
}