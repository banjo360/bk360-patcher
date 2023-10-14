use std::io::SeekFrom;
use std::io::Seek;
use std::fs::File;
use serde::{ Serialize, Deserialize };
use byteorder::{WriteBytesExt, BigEndian};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct Transformations {
    termite: u16,
    croc: u16,
    walrus: u16,
    pumpkin: u16,
    bee: u16,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct HutSpawner {
    actor: u32,
    count: u32,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct MumbosMountain {
    huts: [HutSpawner; 5]
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct Puzzles {
    mm: u8,
    ttc: u8,
    cc: u8,
    bgs: u8,
    fp: u8,
    gv: u8,
    mmm: u8,
    rbb: u8,
    ccw: u8,
    dog: u8,
    dh: u8,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct Data {
    transformations: Transformations,
    mm: MumbosMountain,
    puzzles: Puzzles,
    note_doors: [u16; 12],
}

fn main() -> std::io::Result<()> {
    let args: Vec<String> = std::env::args().collect();

    let mut xex = File::options().read(true).write(true).open(&args[1])?;

    let f = File::open(&args[2])?;
    let data: Data = serde_json::from_reader(f)?;

    update_transformations(&mut xex, &data.transformations)?;
    update_mm(&mut xex, &data.mm)?;
    update_puzzles(&mut xex, &data.puzzles)?;
    update_note_doors(&mut xex, &data.note_doors)?;

    Ok(())
}

fn update_transformations(xex: &mut File, transformations: &Transformations) -> std::io::Result<()> {
    xex.seek(SeekFrom::Start(0x199486))?;
    xex.write_u16::<BigEndian>(transformations.termite)?;
    xex.seek(SeekFrom::Start(0x19946e))?;
    xex.write_u16::<BigEndian>(transformations.croc)?;
    xex.seek(SeekFrom::Start(0x199476))?;
    xex.write_u16::<BigEndian>(transformations.walrus)?;
    xex.seek(SeekFrom::Start(0x19947e))?;
    xex.write_u16::<BigEndian>(transformations.pumpkin)?;
    xex.seek(SeekFrom::Start(0x199466))?;
    xex.write_u16::<BigEndian>(transformations.bee)?;

    Ok(())
}

fn update_mm(xex: &mut File, mm: &MumbosMountain) -> std::io::Result<()> {
    const ADDRESSES: [u64; 5] = [0x45f608, 0x45f63c, 0x45f670, 0x45F6A4, 0x45F740];
    for (id, spawner) in mm.huts.iter().enumerate() {
        xex.seek(SeekFrom::Start(ADDRESSES[id] + 4))?;
        xex.write_u32::<BigEndian>(spawner.actor)?;
        xex.write_u32::<BigEndian>(spawner.count)?;
    }

    Ok(())
}

fn update_puzzles(xex: &mut File, puzzles: &Puzzles) -> std::io::Result<()> {
    xex.seek(SeekFrom::Start(0x462c28))?;
    xex.write_u8(puzzles.mm)?;
    xex.seek(SeekFrom::Start(0x462c28+4))?;
    xex.write_u8(puzzles.ttc)?;
    xex.seek(SeekFrom::Start(0x462c28+8))?;
    xex.write_u8(puzzles.cc)?;
    xex.seek(SeekFrom::Start(0x462c28+12))?;
    xex.write_u8(puzzles.bgs)?;
    xex.seek(SeekFrom::Start(0x462c28+16))?;
    xex.write_u8(puzzles.fp)?;
    xex.seek(SeekFrom::Start(0x462c28+20))?;
    xex.write_u8(puzzles.gv)?;
    xex.seek(SeekFrom::Start(0x462c28+24))?;
    xex.write_u8(puzzles.mmm)?;
    xex.seek(SeekFrom::Start(0x462c28+28))?;
    xex.write_u8(puzzles.rbb)?;
    xex.seek(SeekFrom::Start(0x462c28+32))?;
    xex.write_u8(puzzles.ccw)?;
    xex.seek(SeekFrom::Start(0x462c28+36))?;
    xex.write_u8(puzzles.dog)?;
    xex.seek(SeekFrom::Start(0x462c28+40))?;
    xex.write_u8(puzzles.dh)?;

    Ok(())
}

fn update_note_doors(xex: &mut File, note_doors: &[u16; 12]) -> std::io::Result<()> {
    xex.seek(SeekFrom::Start(0x4670dc))?;
    for notes in note_doors {
        xex.write_u16::<BigEndian>(*notes)?;
    }

    Ok(())
}
