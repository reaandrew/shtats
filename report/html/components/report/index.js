import { h } from 'preact';

export default function Report({children}){
    return (
        <div className="container-fluid">
            <div className="row mb-2">
                <div className="col header">
                    <h1>SHTATS</h1>
                </div>
            </div>
            {children}
        </div>
    )
}