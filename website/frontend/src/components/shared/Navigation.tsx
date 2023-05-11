import Logo from "./Logo";

type Props = {
  currentPage: string;
  setCurrentPage: (page: string) => void;
};

function Navigation({ setCurrentPage, currentPage }: Props) {
  return (
    <div className="container mx-auto flex items-center justify-between py-2">
      <Logo />
      <div className="hidden sm:block">
        <ul className="flex items-center space-x-4">
          <li>
            <button
              className={`text-sm font-medium ${
                currentPage === "home_page" ? "text-blue-500" : "text-gray-500"
              }`}
              onClick={() => setCurrentPage("home_page")}
            >
              Home
            </button>
          </li>
          <li>
            <button
              className={`text-sm font-medium ${
                currentPage === "about_page" ? "text-blue-500" : "text-gray-500"
              }`}
              onClick={() => setCurrentPage("about_page")}
            >
              About
            </button>
          </li>
        </ul>
      </div>
      <div className="sm:hidden">
        <button className="text-sm font-medium text-gray-500">
          {/* Replace this with your burger menu and slider */}
          Menu
        </button>
      </div>
    </div>
  );
}

export default Navigation;