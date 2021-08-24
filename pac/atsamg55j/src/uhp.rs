#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - OHCI Revision Number Register"]
    pub hcrevision: crate::Reg<hcrevision::HCREVISION_SPEC>,
    #[doc = "0x04 - HC Operating Mode Register"]
    pub hccontrol: crate::Reg<hccontrol::HCCONTROL_SPEC>,
    #[doc = "0x08 - HC Command and Status Register"]
    pub hccommandstatus: crate::Reg<hccommandstatus::HCCOMMANDSTATUS_SPEC>,
    #[doc = "0x0c - HC Interrupt and Status Register"]
    pub hcinterruptstatus: crate::Reg<hcinterruptstatus::HCINTERRUPTSTATUS_SPEC>,
    #[doc = "0x10 - HC Interrupt Enable Register"]
    pub hcinterruptenable: crate::Reg<hcinterruptenable::HCINTERRUPTENABLE_SPEC>,
    #[doc = "0x14 - HC Interrupt Disable Register"]
    pub hcinterruptdisable: crate::Reg<hcinterruptdisable::HCINTERRUPTDISABLE_SPEC>,
    #[doc = "0x18 - HC HCCA Address Register"]
    pub hchcca: crate::Reg<hchcca::HCHCCA_SPEC>,
    #[doc = "0x1c - HC Current Periodic Register"]
    pub hcperiodcurrented: crate::Reg<hcperiodcurrented::HCPERIODCURRENTED_SPEC>,
    #[doc = "0x20 - HC Head Control Register"]
    pub hccontrolheaded: crate::Reg<hccontrolheaded::HCCONTROLHEADED_SPEC>,
    #[doc = "0x24 - HC Current Control Register"]
    pub hccontrolcurrented: crate::Reg<hccontrolcurrented::HCCONTROLCURRENTED_SPEC>,
    #[doc = "0x28 - HC Head Bulk Register"]
    pub hcbulkheaded: crate::Reg<hcbulkheaded::HCBULKHEADED_SPEC>,
    #[doc = "0x2c - HC Current Bulk Register"]
    pub hcbulkcurrented: crate::Reg<hcbulkcurrented::HCBULKCURRENTED_SPEC>,
    #[doc = "0x30 - HC Head Done Register"]
    pub hcdonehead: crate::Reg<hcdonehead::HCDONEHEAD_SPEC>,
    #[doc = "0x34 - HC Frame Interval Register"]
    pub hcfminterval: crate::Reg<hcfminterval::HCFMINTERVAL_SPEC>,
    #[doc = "0x38 - HC Frame Remaining Register"]
    pub hcfmremaining: crate::Reg<hcfmremaining::HCFMREMAINING_SPEC>,
    #[doc = "0x3c - HC Frame Number Register"]
    pub hcfmnumber: crate::Reg<hcfmnumber::HCFMNUMBER_SPEC>,
    #[doc = "0x40 - HC Periodic Start Register"]
    pub hcperiodicstart: crate::Reg<hcperiodicstart::HCPERIODICSTART_SPEC>,
    #[doc = "0x44 - HC Low-Speed Threshold Register"]
    pub hclsthreshold: crate::Reg<hclsthreshold::HCLSTHRESHOLD_SPEC>,
    #[doc = "0x48 - HC Root Hub A Register"]
    pub hcrhdescriptora: crate::Reg<hcrhdescriptora::HCRHDESCRIPTORA_SPEC>,
    #[doc = "0x4c - HC Root Hub B Register"]
    pub hcrhdescriptorb: crate::Reg<hcrhdescriptorb::HCRHDESCRIPTORB_SPEC>,
    #[doc = "0x50 - HC Root Hub Status Register"]
    pub hcrhstatus: crate::Reg<hcrhstatus::HCRHSTATUS_SPEC>,
    #[doc = "0x54..0x5c - HC Port 1 Status and Control Register"]
    pub hcrhportstatus: [crate::Reg<hcrhportstatus::HCRHPORTSTATUS_SPEC>; 2],
}
#[doc = "HCREVISION register accessor: an alias for `Reg<HCREVISION_SPEC>`"]
pub type HCREVISION = crate::Reg<hcrevision::HCREVISION_SPEC>;
#[doc = "OHCI Revision Number Register"]
pub mod hcrevision;
#[doc = "HCCONTROL register accessor: an alias for `Reg<HCCONTROL_SPEC>`"]
pub type HCCONTROL = crate::Reg<hccontrol::HCCONTROL_SPEC>;
#[doc = "HC Operating Mode Register"]
pub mod hccontrol;
#[doc = "HCCOMMANDSTATUS register accessor: an alias for `Reg<HCCOMMANDSTATUS_SPEC>`"]
pub type HCCOMMANDSTATUS = crate::Reg<hccommandstatus::HCCOMMANDSTATUS_SPEC>;
#[doc = "HC Command and Status Register"]
pub mod hccommandstatus;
#[doc = "HCINTERRUPTSTATUS register accessor: an alias for `Reg<HCINTERRUPTSTATUS_SPEC>`"]
pub type HCINTERRUPTSTATUS = crate::Reg<hcinterruptstatus::HCINTERRUPTSTATUS_SPEC>;
#[doc = "HC Interrupt and Status Register"]
pub mod hcinterruptstatus;
#[doc = "HCINTERRUPTENABLE register accessor: an alias for `Reg<HCINTERRUPTENABLE_SPEC>`"]
pub type HCINTERRUPTENABLE = crate::Reg<hcinterruptenable::HCINTERRUPTENABLE_SPEC>;
#[doc = "HC Interrupt Enable Register"]
pub mod hcinterruptenable;
#[doc = "HCINTERRUPTDISABLE register accessor: an alias for `Reg<HCINTERRUPTDISABLE_SPEC>`"]
pub type HCINTERRUPTDISABLE = crate::Reg<hcinterruptdisable::HCINTERRUPTDISABLE_SPEC>;
#[doc = "HC Interrupt Disable Register"]
pub mod hcinterruptdisable;
#[doc = "HCHCCA register accessor: an alias for `Reg<HCHCCA_SPEC>`"]
pub type HCHCCA = crate::Reg<hchcca::HCHCCA_SPEC>;
#[doc = "HC HCCA Address Register"]
pub mod hchcca;
#[doc = "HCPERIODCURRENTED register accessor: an alias for `Reg<HCPERIODCURRENTED_SPEC>`"]
pub type HCPERIODCURRENTED = crate::Reg<hcperiodcurrented::HCPERIODCURRENTED_SPEC>;
#[doc = "HC Current Periodic Register"]
pub mod hcperiodcurrented;
#[doc = "HCCONTROLHEADED register accessor: an alias for `Reg<HCCONTROLHEADED_SPEC>`"]
pub type HCCONTROLHEADED = crate::Reg<hccontrolheaded::HCCONTROLHEADED_SPEC>;
#[doc = "HC Head Control Register"]
pub mod hccontrolheaded;
#[doc = "HCCONTROLCURRENTED register accessor: an alias for `Reg<HCCONTROLCURRENTED_SPEC>`"]
pub type HCCONTROLCURRENTED = crate::Reg<hccontrolcurrented::HCCONTROLCURRENTED_SPEC>;
#[doc = "HC Current Control Register"]
pub mod hccontrolcurrented;
#[doc = "HCBULKHEADED register accessor: an alias for `Reg<HCBULKHEADED_SPEC>`"]
pub type HCBULKHEADED = crate::Reg<hcbulkheaded::HCBULKHEADED_SPEC>;
#[doc = "HC Head Bulk Register"]
pub mod hcbulkheaded;
#[doc = "HCBULKCURRENTED register accessor: an alias for `Reg<HCBULKCURRENTED_SPEC>`"]
pub type HCBULKCURRENTED = crate::Reg<hcbulkcurrented::HCBULKCURRENTED_SPEC>;
#[doc = "HC Current Bulk Register"]
pub mod hcbulkcurrented;
#[doc = "HCDONEHEAD register accessor: an alias for `Reg<HCDONEHEAD_SPEC>`"]
pub type HCDONEHEAD = crate::Reg<hcdonehead::HCDONEHEAD_SPEC>;
#[doc = "HC Head Done Register"]
pub mod hcdonehead;
#[doc = "HCFMINTERVAL register accessor: an alias for `Reg<HCFMINTERVAL_SPEC>`"]
pub type HCFMINTERVAL = crate::Reg<hcfminterval::HCFMINTERVAL_SPEC>;
#[doc = "HC Frame Interval Register"]
pub mod hcfminterval;
#[doc = "HCFMREMAINING register accessor: an alias for `Reg<HCFMREMAINING_SPEC>`"]
pub type HCFMREMAINING = crate::Reg<hcfmremaining::HCFMREMAINING_SPEC>;
#[doc = "HC Frame Remaining Register"]
pub mod hcfmremaining;
#[doc = "HCFMNUMBER register accessor: an alias for `Reg<HCFMNUMBER_SPEC>`"]
pub type HCFMNUMBER = crate::Reg<hcfmnumber::HCFMNUMBER_SPEC>;
#[doc = "HC Frame Number Register"]
pub mod hcfmnumber;
#[doc = "HCPERIODICSTART register accessor: an alias for `Reg<HCPERIODICSTART_SPEC>`"]
pub type HCPERIODICSTART = crate::Reg<hcperiodicstart::HCPERIODICSTART_SPEC>;
#[doc = "HC Periodic Start Register"]
pub mod hcperiodicstart;
#[doc = "HCLSTHRESHOLD register accessor: an alias for `Reg<HCLSTHRESHOLD_SPEC>`"]
pub type HCLSTHRESHOLD = crate::Reg<hclsthreshold::HCLSTHRESHOLD_SPEC>;
#[doc = "HC Low-Speed Threshold Register"]
pub mod hclsthreshold;
#[doc = "HCRHDESCRIPTORA register accessor: an alias for `Reg<HCRHDESCRIPTORA_SPEC>`"]
pub type HCRHDESCRIPTORA = crate::Reg<hcrhdescriptora::HCRHDESCRIPTORA_SPEC>;
#[doc = "HC Root Hub A Register"]
pub mod hcrhdescriptora;
#[doc = "HCRHDESCRIPTORB register accessor: an alias for `Reg<HCRHDESCRIPTORB_SPEC>`"]
pub type HCRHDESCRIPTORB = crate::Reg<hcrhdescriptorb::HCRHDESCRIPTORB_SPEC>;
#[doc = "HC Root Hub B Register"]
pub mod hcrhdescriptorb;
#[doc = "HCRHSTATUS register accessor: an alias for `Reg<HCRHSTATUS_SPEC>`"]
pub type HCRHSTATUS = crate::Reg<hcrhstatus::HCRHSTATUS_SPEC>;
#[doc = "HC Root Hub Status Register"]
pub mod hcrhstatus;
#[doc = "HCRHPORTSTATUS register accessor: an alias for `Reg<HCRHPORTSTATUS_SPEC>`"]
pub type HCRHPORTSTATUS = crate::Reg<hcrhportstatus::HCRHPORTSTATUS_SPEC>;
#[doc = "HC Port 1 Status and Control Register"]
pub mod hcrhportstatus;
