library IEEE;
use IEEE.STD_LOGIC_1164.ALL;

//declaração de variáveis (portas lógicas)
//entity é a descrição do componente que vai usado no projeto
entity decoder_3to8 is
 port (
    //tem duas portas, a A de 3bits e a Y de 8bits
    A : in std_logic_vector (2 downto 0);
    //como Y é out, é uma saída 
    Y : out std_logic_vector (7 downto 0);
 );
end entity decoder_3to8;

//começa a arquitetura "Comportamento" do componente
//literalmente descreve o comportamento do decodificador (entidade)
architecture Comportamento of decoder_3to8 is
begin
    //inicia um processo que é sensível à mudança no sinal de entrada de A
    //sempre que o valor de A mudar, o código do processo vai ser executado
    process(A)
    begin 
        //estrutura de seleção case
        //verifica o valor de A e executa o código correspondente
        case A is
            //quando as entradas de A forem "000" 
            when "000" =>
                // a saída de Y vai ser "10000000"
                Y <= "10000000";
            when "001" =>
                Y <= "01000000";
            when "010" =>
                Y <= "00100000";
            when "011" =>
                Y <= "00010000";
            when "100" =>
                Y <= "00001000";
            when "101" =>
                Y <= "00000100";
            when "110" =>
                Y <= "00000010";
            when "111" =>
                Y <= "00000001";
            //quando o valor de A não for um definido pela tabela verdade
            when others =>
                //Y é "00000000"
                Y <= "00000000";
        end case;
    end process;
end architecture Comportamento;
