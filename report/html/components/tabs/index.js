import { h } from 'preact';
import {useEffect, useState} from "preact/compat";

export default function Tabs({children}){

    const [active, setActive] = useState(children[0]);

    function activate(x){
        setActive(x);
        setTimeout(
            ()=>{window.dispatchEvent(new Event('resize'));},
            1
        );
    }

    return <div class={"row"}>
        <div className="d-flex align-items-start p-0 m-0">
            <div className="nav flex-column nav-pills col-2 pe-1" id="v-pills-tab" role="tablist" aria-orientation="vertical">
                {children.map(x=>(
                    <button className={`nav-link ${active == x ? 'active': null}`} id={`v-pills-${x.props.label}-tab`} data-bs-toggle="pill"
                            data-bs-target={`#v-pills-${x.props.label}`} type="button" role="tab" aria-controls={`v-pills-${x.props.label}`}
                            aria-selected="true" onClick={e => activate(x)}>{x.props.label}
                    </button>
                ))}
            </div>
            <div className="tab-content col-10" id="v-pills-tabContent">
                {children.map(x=>(
                    <div className={`tab-pane fade ${active == x ? 'show active': null}`} id={`v-pills-${x.props.label}`} role="tabpanel"
                         aria-labelledby={`v-pills-${x.props.label}-tab`}>
                        {x}
                    </div>
                ))}

            </div>
        </div>

    </div>
}