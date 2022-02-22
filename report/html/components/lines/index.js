import {h} from 'preact';
import * as echarts from "echarts";
import {useEffect} from "preact/compat";
import LinesAddedDeleted from "../lines_added_deleted";
import LinesCumulative from "../lines_cumulative";


export default function Lines({data}) {
    return (
        <div>
            <div className="row">
                <div className="col">
                    <LinesAddedDeleted data={data}/>
                </div>
            </div>
            <div className="row">
                <div className="col">
                    <LinesCumulative data={data}/>
                </div>
            </div>
        </div>
    )
}