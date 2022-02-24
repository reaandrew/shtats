import {h} from 'preact';
import {useEffect} from "preact/compat";
import * as echarts from "echarts";

export default function FilesSummary({data}){

    function summarise_file_operations(data){
        return data.reduce((prev, curr) => {
            prev.added += curr[1];
            prev.deleted += curr[2];
            prev.modified += curr[3];
            prev.renamed += curr[4];
            return prev;
        }, {added:0, deleted:0, modified:0, renamed:0})
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
                    color: ['#2DA1EF', '#00bef4', '#00d6de', '#18e8b6'],
                    name: '',
                    type: 'pie',
                    radius: '50%',
                    data: [
                        {value: data.added, name: 'Added'},
                        {value: data.deleted, name: 'Deleted'},
                        {value: data.added, name: 'Modified'},
                        {value: data.deleted, name: 'Renamed'},
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
        window.addEventListener('resize', function(){
            if(myChart != null && myChart != undefined){
                myChart.resize();
            }
        });
    }

    useEffect(() => {
        pie_chart("files_summary", summarise_file_operations((data)))
    }, [])

    return (
        <div className="row">
            <div className="col">
                <h2>Files Summary</h2>
                <div id="files_summary" className="col chart"/>
            </div>
        </div>
    )
}