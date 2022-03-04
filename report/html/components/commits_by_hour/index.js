import {h} from 'preact';
import * as echarts from "echarts";
import {useEffect} from "preact/compat";

export default function CommitsByHour({data}){
    const max_size = 55;
    function punch_chart(data, element){
        let max_commit = Math.max.apply(Math, data.map((x) => x[2]));
        let adjusted_value = (value) => {
            return (value/max_commit) * max_size;
        }

        var chartDom = document.getElementById(element);
        var myChart = echarts.init(chartDom);
        var option;

// prettier-ignore
        const hours = [
            '12a', '1a', '2a', '3a', '4a', '5a', '6a',
            '7a', '8a', '9a', '10a', '11a',
            '12p', '1p', '2p', '3p', '4p', '5p',
            '6p', '7p', '8p', '9p', '10p', '11p'
        ];
// prettier-ignore
        const days = [
            'Sunday', 'Monday', 'Tuesday', 'Wednesday', 'Thursday', 'Friday', 'Saturday'
        ];
// prettier-ignore
        const title = [];
        const singleAxis = [];
        const series = [];
        days.forEach(function (day, idx) {
            title.push({
                textBaseline: 'middle',
                top: ((idx + 0.5) * 100) / 7 + '%',
                text: day
            });
            singleAxis.push({
                left: 150,
                type: 'category',
                boundaryGap: false,
                data: hours,
                top: (idx * 100) / 7 + 5 + '%',
                height: 100 / 7 - 10 + '%',
                axisLabel: {
                    interval: 2
                }
            });
            series.push({
                singleAxisIndex: idx,
                coordinateSystem: 'singleAxis',
                type: 'scatter',
                data: [],
                symbolSize: function (dataItem) {
                    return adjusted_value(dataItem[1]);
                }
            });
        });
        data.forEach(function (dataItem) {
            series[dataItem[0]].data.push([dataItem[1], dataItem[2]]);
        });
        option = {
            tooltip: {
                position: 'top'
            },
            title: title,
            singleAxis: singleAxis,
            series: series
        };

        option && myChart.setOption(option);

        window.addEventListener('resize', function(){
            if(myChart != null && myChart != undefined){
                myChart.resize();
            }
        });
    }


    useEffect(() => {
        punch_chart(data, "commits_by_hour");
    }, []);

    return (
        <div className="row">
            <div className="col">
                <h2>Commits By Hour</h2>
                <div id="commits_by_hour" className="col chart"/>
            </div>
        </div>
    )
};