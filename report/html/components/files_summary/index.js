import {h} from 'preact';
import {useEffect} from "preact/compat";
import * as echarts from "echarts";

export default function FilesSummary({data}) {

    function summarise_file_operations(data) {
        return data.reduce((prev, curr) => {
            prev.added += curr[1];
            prev.modified += curr[2];
            prev.deleted += curr[3];
            prev.renamed += curr[4];
            prev.copied += curr[5];
            prev.type_changed += curr[6];
            prev.unmerged += curr[7];
            prev.unknown += curr[8];
            prev.pairing_broken += curr[9];
            return prev;
        }, {
            added: 0,
            modified: 0,
            deleted: 0,
            renamed: 0,
            copied: 0,
            type_changed: 0,
            unmerged: 0,
            unknown: 0,
            pairing_broken: 0
        })
    }

    function pie_chart(element, data) {
        const chartDom = document.getElementById(element);
        const myChart = echarts.init(chartDom);
        let option;

        let chart_data = [
            {value: data.added, name: 'Added'},
            {value: data.modified, name: 'Modified'},
            {value: data.deleted, name: 'Deleted'},
            {value: data.renamed, name: 'Renamed'},
            {value: data.copied, name: 'Copied'},
            {value: data.type_changed, name: 'Type Changed'},
            {value: data.unmerged, name: 'Unmerged'},
            {value: data.unknown, name: 'Unknown'},
            {value: data.pairing_broken, name: 'Pairing Broken'},
        ].filter(x=>x.value > 0);


        option = {
            title: {
                show: false
            },
            tooltip: {
                trigger: 'item'
            },
            legend: {
                show: false
            },
            series: [
                {
                    //color: ['#2DA1EF', '#00bef4', '#00d6de', '#18e8b6'],
                    name: '',
                    type: 'pie',
                    radius: '50%',
                    data: chart_data,
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
        window.addEventListener('resize', function () {
            if (myChart != null && myChart != undefined) {
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