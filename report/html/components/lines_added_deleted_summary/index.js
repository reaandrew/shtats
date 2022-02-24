import {h} from 'preact';
import {useEffect} from "preact/compat";
import * as echarts from "echarts";

export default function LinesAddedDeletedSummary({data}){

    function summarise_added_deleted(data){
        return data.reduce((prev, curr) => {
            prev.added += curr[1];
            prev.deleted += curr[2];
            return prev;
        }, {added:0, deleted:0})
    }

    function pie_chart(element, data) {
        const chartDom = document.getElementById(element);
        const myChart = echarts.init(chartDom);
        let option;

        option = {
            title: {
                show:false
            },
            tooltip: {
                trigger: 'item'
            },
            legend: {
                show:false
            },
            series: [
                {
                    color: ['#2DA1EF','#fd0e35'],
                    name: '',
                    type: 'pie',
                    radius: '50%',
                    data: [
                        {value: data.added, name: 'Added'},
                        {value: data.deleted, name: 'Deleted'},
                    ],
                    emphasis: {
                        itemStyle: {
                            shadowBlur: 10,
                            shadowOffsetX: 0,
                            shadowColor: 'rgba(0, 0, 0, 0.5)'
                        }
                    }
                }
            ]
        };

        option && myChart.setOption(option);
    }

    useEffect(() => {
        pie_chart("lines_added_deleted_summary", summarise_added_deleted((data)))
    }, [])

    return (
        <div className="row">
            <div className="col">
                <h2>Lines Summary</h2>
                <div id="lines_added_deleted_summary" className="col chart"/>
            </div>
        </div>
    )
}