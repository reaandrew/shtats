import {h} from 'preact';
import * as echarts from "echarts";
import {useEffect} from "preact/compat";
import UserSummary from "../users_summary";

export default function UserCollection({data}) {

    useEffect(() => {

    }, []);

    function getUserSummaries(){
        return data.map((x, index)=> (
            <div className="col-xs-12 col-sm-12 col-md-4">
                <UserSummary data={x} id={"user_summary_"+index} />
            </div>
        ));
    }

    return (
        <div>
            <div className="row">
                <div className="col">
                    <h2>Top 25 Users Summary</h2>
                </div>
            </div>
            <div className="row">
                {getUserSummaries()}
            </div>
        </div>
    )
}