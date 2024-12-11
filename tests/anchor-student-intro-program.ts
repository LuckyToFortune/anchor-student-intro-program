import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { AnchorStudentIntroProgram } from "../target/types/anchor_student_intro_program";
import { expect } from "chai";

describe("anchor-student-intro-program", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace
    .AnchorStudentIntroProgram as Program<AnchorStudentIntroProgram>;
  const userWallet = anchor.workspace.AnchorStudentIntroProgram.provider.wallet;

  const student = {
    name: "name",
    message: "message",
  };

  const realloc = {
    name: "realloc",
    message: "realloc",
  };

  const [studentIntroPda] = anchor.web3.PublicKey.findProgramAddressSync(
    [userWallet.publicKey.toBuffer()],
    program.programId
  );

  it("Add Student Intro", async () => {
    const tx = await program.methods
      .addStudentIntro(student.name, student.message)
      .accounts({
        studentIntro: studentIntroPda,
        student: userWallet.publicKey,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .rpc();

    const account = await program.account.studentInfo.fetch(studentIntroPda);
    expect(student.name === account.name);
    expect(student.message === account.message);
  });

  it("Update Student Intro", async () => {
    const tx = await program.methods
      .updateStudentIntro(realloc.name, realloc.message)
      .accounts({
        studentIntro: studentIntroPda,
        student: userWallet.publicKey,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .rpc();

    const account = await program.account.studentInfo.fetch(studentIntroPda);
    expect(realloc.name === account.name);
    expect(realloc.message === account.message);
  });

  it("Close Account", async () => {
    const tx = await program.methods
      .close()
      .accounts({
        studentIntro: studentIntroPda,
        student: userWallet.publicKey,
      })
      .rpc();

    try {
      const account = await program.account.studentInfo.fetch(studentIntroPda);
    } catch (error) {
      console.log("\nFailed fetching account: ", error);
    }
  });
});
