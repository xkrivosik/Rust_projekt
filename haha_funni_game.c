#include <stdio.h>
#include <string.h>
#include <stdlib.h>

//mas mali kokot
//nie ty
//bombaklat
void print_board(char board[3][3]) {
    printf("  1   2   3\n");
    printf("A %c | %c | %c \n", board[0][0], board[0][1], board[0][2]);
    printf(" ---|---|---\n");
    printf("B %c | %c | %c \n", board[1][0], board[1][1], board[1][2]);
    printf(" ---|---|---\n");
    printf("C %c | %c | %c \n", board[2][0], board[2][1], board[2][2]);
}
int main()
{
    char press;
    char name[50];
    int dumbass=0;
    char answear[4];
    char play[2];
    char play2[2];
    char board[3][3] = {
        {' ', ' ', ' '},
        {' ', ' ', ' '},
        {' ', ' ', ' '}
    };
    int gami=0;
    printf("|************|\n");
    printf("| Funni game |\n");
    printf("|************|\n");
    printf("  /to proceed press enter/");
    scanf("%c",&press);
    printf("\n( x - 0) <(Hey.....)");
    scanf("%c",&press);
    printf("\n( x o 0) <(My name is Gami! Not to be confused with Calci)");
    scanf("%c",&press);
    printf("\n( x - 0) <(That's my cousin and she is sooo annoying...)");
    scanf("%c",&press);
    printf("\n( x o 0) <(Aaaaand, who are you supposed to be?)\n\nEnter your name:");
    scanf("%s",&name);
    if(strcmp(name,"Calci")==0){
        printf("\n( x _ o)<(Fuck, it's you!)");
        return 0;
    }
    else if(strcmp(name,"Gami")==0){
        printf("\n( x _ 0)<(Stop, bullshitting)");
        scanf("%c",&press);
        scanf("%c",&press);
        printf("\n( x _ - )<(Okay, if you don't want to tell me your real name, I'll just call you 'dumbass')");
        name[0]='d';
        name[1]='u';
        name[2]='m';
        name[3]='b';
        name[4]='a';
        name[5]='s';
        name[6]='s';
        dumbass=1;
    }
    else{
       printf("\n( x _ -) <(That's a dumb name)",name);
       scanf("%c",&press);
    }
    scanf("%c",&press);
    if(dumbass==1){
        printf("\n( x - 0) <(Allright dumbass, wanna play a game?)(YES/NO):");
        scanf("%s",&answear);
    }
    else{
        printf("\n( x - 0) <(Allright %s, wanna play a game?)(YES/NO):",name);
        scanf("%s",&answear);
    }
    printf("\n( x o ^) <(I don't care what your answear is, we are playing...)");
    scanf("%c",&press);
    scanf("%c",&press);
    printf("\n( x - 0) <(Allright, listen up, we are going to play a game called...)");
    scanf("%c",&press);
    printf("\n( x w 0) <(*Tic Tac Toe*)");
    scanf("%c",&press);
    printf("\n( x . -) <(You know tic tac toe, right?)");
    scanf("%c",&press);
    printf("\n( x - 0) <(Aaaah, it doesn't matter, you'll get it...)");
    scanf("%c",&press);
    printf("\n( x o 0) <(Okay, I'm going to be X and I'll start, beacuse...)");
    scanf("%c",&press);
    printf("\n( x w ^) <(Oviously I do)");
    scanf("%c",&press);
    printf("\n( x - 0) <(So, here is the board)\n\n");
    print_board(board);
    scanf("%c",&press);
    printf("\n( x o 0) <(I place my X on B2, your turn!)\n\n");
    board[1][1] ='X';
    print_board(board);
    scanf("%c",&press);
    printf("\nYour play(A1/A2/A3/B1/B3/C1/C2/C3) /if you type anything else your turn is skipped ;)/:");
    scanf("%s",&play);
    if(strcmp(play,"A1")==0){
        board[0][0]='O';
    }
    if(strcmp(play,"A2")==0){
        board[0][1]='O';
    }
    if(strcmp(play,"A3")==0){
        board[0][2]='O';
    }
    if(strcmp(play,"B1")==0){
        board[1][0]='O';
    }
    if(strcmp(play,"B3")==0){
        board[1][2]='O';
    }
    if(strcmp(play,"C1")==0){
        board[2][0]='O';
    }
    if(strcmp(play,"C2")==0){
        board[2][1]='O';
    }
    if(strcmp(play,"C3")==0){
        board[2][2]='O';
    }
    while(strcmp(play,"B2")==0){
        printf("\nYou can't do that!\n");
        printf("\nYour play(A1/A2/A3/B1/B3/C1/C2/C3):");
        scanf("%s",&play);
        if(strcmp(play,"A1")==0){
        board[0][0]='O';
        }
        if(strcmp(play,"A2")==0){
            board[0][1]='O';
        }
        if(strcmp(play,"A3")==0){
            board[0][2]='O';
        }
        if(strcmp(play,"B1")==0){
            board[1][0]='O';
        }
        if(strcmp(play,"B3")==0){
            board[1][2]='O';
        }
        if(strcmp(play,"C1")==0){
            board[2][0]='O';
        }
        if(strcmp(play,"C2")==0){
            board[2][1]='O';
        }
        if(strcmp(play,"C3")==0){
            board[2][2]='O';
        }
    }
    printf("\n");
    print_board(board);
    scanf("%c",&press);
    printf("\n( x _ 0) <(Hmmmm)");
    scanf("%c",&press);
    if((strcmp(play,"A1")!=0)&&(strcmp(play,"C3")!=0)){
        printf("\n( x o 0) <(I play A1)\n\n");
        board[0][0]='X';
        print_board(board);
        scanf("%c",&press);
        gami++;
        if(strcmp(play,"A2")==0){
            printf("\nYour play(A3/B1/B3/C1/C2/C3):");
        }
        if(strcmp(play,"A3")==0){
            printf("\nYour play(A2/B1/B3/C1/C2/C3):");
        }
        if(strcmp(play,"B1")==0){
            printf("\nYour play(A2/A3/B3/C1/C2/C3):");
        }
        if(strcmp(play,"B3")==0){
            printf("\nYour play(A2/A3/B1/C1/C2/C3):");
        }
        if(strcmp(play,"C1")==0){
            printf("\nYour play(A2/A3/B1/B3/C2/C3):");
        }
        if(strcmp(play,"C2")==0){
            printf("\nYour play(A2/A3/B1/B3/C1/C3):");
        }
        scanf("%s",&play2);
    }
    if((strcmp(play,"A3")!=0)&&(strcmp(play,"C1")!=0)&&gami==0){
        printf("\n( x o 0) <(I play A3)\n\n");
        board[0][2]='X';
        print_board(board);
        scanf("%c",&press);
        if(strcmp(play,"A1")==0){
            printf("\nYour play(A2/B1/B3/C1/C2/C3):");
        }
        if(strcmp(play,"A2")==0){
            printf("\nYour play(A1/B1/B3/C1/C2/C3):");
        }
        if(strcmp(play,"B1")==0){
            printf("\nYour play(A1/A2/B3/C1/C2/C3):");
        }
        if(strcmp(play,"B3")==0){
            printf("\nYour play(A1/A2/B1/C1/C2/C3):");
        }
        if(strcmp(play,"C2")==0){
            printf("\nYour play(A1/A2/B1/B3/C1/C3):");
        }
        if(strcmp(play,"C3")==0){
            printf("\nYour play(A1/A2/B1/B3/C1/C2):");
        }
        scanf("%s",&play2);
    }
    printf("\n( x o 0) <(Woah! Looks like something went wrong...)");
    scanf("%c",&press);
    scanf("%c",&press);
    printf("\n( x - 0) <(Your play, got lost somehow...)");
    scanf("%c",&press);
    printf("\n( x _ -) <(Did you type it correctly?)");
    scanf("%c",&press);
    printf("\n( x v ^) <(Well anyways, guess it's my turn now!)");
    scanf("%c",&press);
    if(gami==0){
        printf("\n( x u 0) <(I play C1)\n\n");
        board[2][0]='X';
        print_board(board);
        scanf("%c",&press);
    }
    if(gami==1){
        printf("\n( x u 0) <(I play C3)\n\n");
        board[2][2]='X';
        print_board(board);
        scanf("%c",&press);
    }
    printf("\n( x - 0) <(And that's game partner)");
    scanf("%c",&press);
    printf("\n( x . -) <(I have to say you were quite a formidable player...)");
    scanf("%c",&press);
    printf("\n( x . 0) <(What?)");
    scanf("%c",&press);
    printf("\n( x - 0) <(Why are you looking at me like that?)");
    scanf("%c",&press);
    printf("\n( x o 0) <(You think I cheated?)");
    scanf("%c",&press);
    printf("\n( x _ 0) <(Wait what was your name again?)");
    scanf("%c",&press);
    if(strcmp(name,"Andy")==0){
        printf("\n( x _ -) <(Sorry my creator, I forgot I was playing against you)");
        scanf("%c",&press);
        printf("\n( x _ -) <(You are right, I cheated....)");
        scanf("%c",&press);
        printf("\n( x _ -) <(You are the greatest player in the history of mankind and noone can beat you)");
        scanf("%c",&press);
        printf("\n/Funni game shutting down/");
        scanf("%c",&press);
    }
    else{
        printf("\n( x v -) <(Aaaaah yes %s)",name);
        scanf("%c",&press);
        printf("\n( x o 0) <(Listen here %s....)",name);
        scanf("%c",&press);
        printf("\n( x v 0) <(You are just shit at games, get used to it)");
        scanf("%c",&press);
        printf("\n( x _ -) <(Now get out of my sight, I'm going to celebrate my victory with a nap)");
        scanf("%c",&press);
        printf("\n/Funni game shutting down/");
        scanf("%c",&press);
    }
}
