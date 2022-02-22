import {h} from 'preact';
import * as echarts from "echarts";
import {useEffect} from "preact/compat";
import CommitsByDay from "../commits_by_day";
import CommitsByWeekday from "../commits_by_weekday";
import CommitsByMonth from "../commits_by_month";
import CommitsCumulative from "../commits_cumulative";

export default function Commits({data}) {
    return (
        <div>
            <div className="row">
                <div className="col">
                    <CommitsByDay data={data}/>
                </div>
            </div>
            <div className="row">
                <div className="col">
                    <CommitsByWeekday data={data} />
                </div>
                <div className="col">
                    <CommitsByMonth data={data} />
                </div>
            </div>
            <div className="row">
                <div className="col">
                    <CommitsCumulative data={data}/>
                </div>
            </div>
        </div>
    )
}