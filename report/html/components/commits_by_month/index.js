import {h} from 'preact';
import * as echarts from "echarts";
import {useEffect} from "preact/compat";

export default function CommitsByMonth({data}){
    const month_names_short=  ['Jan', 'Feb', 'Mar', 'Apr', 'May', 'Jun', 'Jul', 'Aug', 'Sep', 'Oct', 'Nov', 'Dec'];

    function total_commits_by_month_model(){
        let obj = total_commits_by_day_model.map(x=>[new Date(x[0]).getMonth(),x[1]]).reduce((prev,curr) => {
            if (prev[curr[0]] === undefined){
                prev[curr[0]] = curr[1]
            }else{
                prev[curr[0]]+= curr[1]
            }
            return prev
        }, {});
        return Object.keys(obj).map((key) => [Number(key), obj[key]]);
    }

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
        window.addEventListener('resize', function () {
            if (myChart != null) {
                myChart.resize();
            }
        });
    }

    useEffect(() => {
        let data = total_commits_by_month_model();
        let sorted_data = [...data].sort((a,b) => a[0]-b[1]);
        let categories = sorted_data.map(x => month_names_short[x[0]]);
        let values = sorted_data.map(x => x[1]);
        bar_category("total_commits_by_month", categories, values);
    }, []);

    return (
        <div className="row">
            <div className="col">
                <h2>Commits by month</h2>
                <div id="total_commits_by_month" className="col chart"/>
            </div>
        </div>
    );
}