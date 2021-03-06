import {h} from 'preact';
import * as echarts from "echarts";
import {useEffect} from "preact/compat";
import CommitsByDay from "../commits_by_day";
import CommitsByWeekday from "../commits_by_weekday";
import CommitsByMonth from "../commits_by_month";
import CommitsCumulative from "../commits_cumulative";
import CommitsByHour from "../commits_by_hour";

export default function CommitsCollection({data, punch_data}) {
    return (
        <div>
            <div className="row">
                <div className="col">
                    <CommitsByDay data={data}/>
                </div>
            </div>
            <div className="row">
                <div className="col-6">
                    <CommitsByWeekday data={data} />
                </div>
                <div className="col-6">
                    <CommitsByMonth data={data} />
                </div>
            </div>
            <div className="row">
                <div className="col">
                    <CommitsCumulative data={data}/>
                </div>
            </div>
            <div className="row">
                <div className="col">
                    <CommitsByHour data={punch_data} />
                </div>
            </div>
        </div>
    )
}