import {h} from 'preact';
import * as echarts from "echarts";
import {useEffect} from "preact/compat";

export default function CommitsByWeekday({data}) {
    function total_commits_by_weekday_model(){
        let obj = total_commits_by_day_model.map(x=>[new Date(x[0]).getDay(),x[1]]).reduce((prev,curr) => {
            if (prev[curr[0]] === undefined){
                prev[curr[0]] = curr[1]
            }else{
                prev[curr[0]]+= curr[1]
            }
            return prev
        }, {0:0,1:0,2:0,3:0,4:0,5:0,6:0});
        return Object.keys(obj).map((key) => [Number(key), obj[key]])
    };
    function bar_category(element, categories, data) {
        var chartDom = document.getElementById(element);
        var myChart = echarts.init(chartDom);
        var option;

        option = {
            xAxis: {
                type: 'category',
                data: categories
            },
            yAxis: {
                type: 'value'
            },
            series: [
                {
                    data: data,
                    type: 'bar',
                    showBackground: true,
                    backgroundStyle: {
                        color: 'rgba(180, 180, 180, 0.2)'
                    },
                    itemStyle: {
                        color: '#2DA1EF'
                    }
                }
            ]
        };

        option && myChart.setOption(option);
    }

    useEffect(() => {
        let data = total_commits_by_weekday_model();
        let categories = ['Sun', 'Mon', 'Tue', 'Wed', 'Thu', 'Fri', 'Sat'];
        let values = data.map(x => x[1]);
        bar_category("total_commits_by_weekday", categories, values);

    }, []); //
    return (
        <div className="row">
            <div className="col">
                <h2>Commits by weekday</h2>
                <div id="total_commits_by_weekday" className="chart"/>
            </div>
        </div>
    );
}