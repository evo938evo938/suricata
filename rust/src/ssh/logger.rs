/* Copyright (C) 2018 Open Information Security Foundation
 *
 * You can copy, redistribute or modify this Program under the terms of
 * the GNU General Public License version 2 as published by the Free
 * Software Foundation.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * version 2 along with this program; if not, write to the Free Software
 * Foundation, Inc., 51 Franklin Street, Fifth Floor, Boston, MA
 * 02110-1301, USA.
 */

use super::ssh::SSHTransaction;
use crate::json::*;
use std;

fn log_ssh(tx: &SSHTransaction) -> Option<Json> {
    let js = Json::object();
    let cjs = Json::object();
    cjs.set_string(
        "proto_version",
        std::str::from_utf8(&tx.cli_hdr.protover).unwrap(),
    );
    cjs.set_string(
        "software_version",
        std::str::from_utf8(&tx.cli_hdr.swver).unwrap(),
    );
    let sjs = Json::object();
    sjs.set_string(
        "proto_version",
        std::str::from_utf8(&tx.srv_hdr.protover).unwrap(),
    );
    sjs.set_string(
        "software_version",
        std::str::from_utf8(&tx.srv_hdr.swver).unwrap(),
    );
    js.set("client", cjs);
    js.set("server", sjs);
    return Some(js);
}

#[no_mangle]
pub extern "C" fn rs_ssh_log_json(tx: *mut std::os::raw::c_void) -> *mut JsonT {
    let tx = cast_pointer!(tx, SSHTransaction);
    match log_ssh(tx) {
        Some(js) => js.unwrap(),
        None => std::ptr::null_mut(),
    }
}