import {h} from 'preact';
import {useEffect} from "preact/compat";
import * as echarts from "echarts";

export default function LinesAddedDeletedSummaryTable({data}){

    useEffect(() => {

    }, [])

    return (
        <div className="row">
            <div className="col">
                <h2>Lines Averages</h2>
                <table className={"table mt-4"}>
                    <thead>
                        <tr className={"shtats-blue"}>
                            <th>Average by:</th>
                            <th>hour</th>
                            <th>day</th>
                            <th>week</th>
                            <th>month</th>
                            <th>commit</th>
                        </tr>
                    </thead>
                    <tbody>
                        <tr>
                            <td>Lines Added</td>
                            <td>{data.hour.added}</td>
                            <td>{data.day.added}</td>
                            <td>{data.week.added}</td>
                            <td>{data.month.added}</td>
                            <td>{data.commit.added}</td>
                        </tr>
                        <tr>
                            <td>Lines Deleted</td>
                            <td>{data.hour.deleted}</td>
                            <td>{data.day.deleted}</td>
                            <td>{data.week.deleted}</td>
                            <td>{data.month.deleted}</td>
                            <td>{data.commit.deleted}</td>
                        </tr>
                        <tr>
                            <td>Churn</td>
                            <td>{data.hour.churn}</td>
                            <td>{data.day.churn}</td>
                            <td>{data.week.churn}</td>
                            <td>{data.month.churn}</td>
                            <td>{data.commit.churn}</td>
                        </tr>
                    </tbody>
                </table>
            </div>
        </div>
    )
}