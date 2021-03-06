import {h} from 'preact';
import * as echarts from "echarts";
import {useEffect} from "preact/compat";
import LinesAddedDeleted from "../lines_added_deleted";
import LinesCumulative from "../lines_cumulative";
import LinesAddedDeletedSummary from "../lines_added_deleted_summary";
import LinesAddedDeletedSummaryTable from "../lines_added_deleted_summary_table";
import LinesAddedDeletedByLanguage from "../lines_added_deleted_by_language";


export default function LinesCollection({data}) {
    return (
        <div>
            <div className="row">
                <div className="col">
                    <LinesAddedDeleted data={data.total_lines_by_day}/>
                </div>
            </div>
            <div className="row">
                <div className="col">
                    <LinesCumulative data={data.total_lines_by_day}/>
                </div>
            </div>
            <div className="row">
                <div className="col">
                    <LinesAddedDeletedByLanguage data={data.avg_line_stats_by_file_extension} />
                </div>
            </div>
            <div className="row">
                <div className="col-4">
                    <LinesAddedDeletedSummaryTable data={data.avg_line_stats} />
                </div>
                <div className="col-8">
                    <LinesAddedDeletedSummary data={data.total_lines_by_day} />
                </div>
            </div>
        </div>
    )
}