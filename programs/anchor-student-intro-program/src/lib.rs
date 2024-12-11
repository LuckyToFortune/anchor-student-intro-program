use anchor_lang::prelude::*;

mod constants;
use constants::*;

declare_id!("F93GhPc7jhyumu3oEy5Vnej9UKHx4imaPPVDGcNE6Gdn");

#[program]
pub mod anchor_student_intro_program {
    use super::*;

    pub fn add_student_intro(
        ctx: Context<AddStudentIntro>,
        name: String,
        message: String
    ) -> Result<()> {
        msg!("Student Intro Account Created");
        msg!("Name: {}", name);
        msg!("Message: {}", message);

        ctx.accounts.student_intro.set_inner(StudentInfo {
            student: ctx.accounts.student.key(),
            name,
            message,
            bump: ctx.bumps.student_intro
        });

        Ok(())
    }

    pub fn update_student_intro (
        ctx: Context<UpdateStudentIntro>,
        name: String,
        message: String
    ) -> Result<()> {
        msg!("Updating Student Intro Account");
        msg!("Name: {}", name);
        msg!("Message: {}", message);

        let student_intro = &mut ctx.accounts.student_intro;
        student_intro.student = ctx.accounts.student.key();
        student_intro.name = name;
        student_intro.message = message;

        Ok(())
    }

    pub fn close(_ctx: Context<Close>) -> Result<()> {
        Ok(())
    }
    
}

#[derive(Accounts)]
#[instruction(name:String, message:String)]
pub struct AddStudentIntro<'info> {
    #[account(
        init,
        seeds=[student.key().as_ref()],
        bump,
        payer = student,
        space = StudentInfo::INIT_SPACE + name.len() + message.len()
    )]
    pub student_intro: Account<'info, StudentInfo>,
    #[account(mut)]
    pub student: Signer<'info>,
    pub system_program: Program<'info, System>
}

#[derive(Accounts)]
#[instruction(name:String, message:String)]
pub struct UpdateStudentIntro<'info> {
    #[account(
        mut,
        seeds = [student.key().as_ref()],
        bump = student_intro.bump,
        realloc = StudentInfo::INIT_SPACE + name.len() + message.len(),
        realloc::payer = student,
        realloc::zero = false
    )]
    pub student_intro: Account<'info, StudentInfo>,
    #[account(mut)]
    pub student: Signer<'info>,
    pub system_program: Program<'info, System>
}

#[derive(Accounts)]
pub struct Close<'info> {
    #[account(
        mut,
        seeds = [student.key().as_ref()],
        bump = student_intro.bump,
        close = student
    )]
    student_intro: Account<'info, StudentInfo>,
    #[account(mut)]
    student: Signer<'info>
}

#[account]
pub struct StudentInfo {
    pub student: Pubkey,
    pub name: String,
    pub message: String,
    pub bump: u8
}

impl Space for StudentInfo {
    const INIT_SPACE: usize = ANCHOR_DISCRIMINATOR + PUBKEY_SIZE + STRING_PREFIX_SIZE + STRING_PREFIX_SIZE + BUMP_SIZE;
}